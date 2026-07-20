//! Golden snapshot tests for generated output.
//!
//! Each config generates a small loader from the bundled specs and compares
//! the generated .h/.c (with the preamble comment stripped — its content
//! varies with gloam version and pinned upstream commits) byte-for-byte
//! against checked-in golden files under `tests/golden/<config>/`.
//!
//! These exist so template and generator refactors that are supposed to be
//! output-neutral can be *proven* neutral by `cargo test`, without
//! regenerating and diffing full loader trees by hand.
//!
//! When output changes deliberately — or a bundled spec update (Monday
//! `cargo xtask bundle`) changes resolved content — re-bless and review:
//!
//!   GLOAM_BLESS=1 cargo test --test golden
//!   git diff tests/golden/     # the delta IS the review artifact

mod common;
use common::generate;

use std::path::PathBuf;

fn golden_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
}

fn blessing() -> bool {
    std::env::var_os("GLOAM_BLESS").is_some()
}

/// Strip the leading block-comment preamble: golden comparison starts after
/// the first `*/`.  The preamble embeds the gloam version and upstream
/// commit describes, which vary without the templates changing.
fn strip_preamble(content: &str) -> &str {
    match content.find("*/") {
        Some(pos) => content[pos + 2..].trim_start_matches('\n'),
        None => content,
    }
}

/// Cheap structural invariant: every `#if`/`#ifdef`/`#ifndef` in the
/// generated file must have a matching `#endif`.
fn assert_preprocessor_balance(name: &str, rel: &str, content: &str) {
    let opens = content
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("#if") // covers #if, #ifdef, #ifndef
        })
        .count();
    let closes = content
        .lines()
        .filter(|l| l.trim_start().starts_with("#endif"))
        .count();
    assert_eq!(
        opens, closes,
        "[{name}/{rel}] unbalanced preprocessor conditionals: {opens} #if* vs {closes} #endif"
    );
}

/// Structural invariant: generated files end with exactly one newline —
/// no blank lines before EOF, no missing final newline.
fn assert_single_trailing_newline(name: &str, rel: &str, content: &str) {
    assert!(
        content.ends_with('\n') && !content.ends_with("\n\n"),
        "[{name}/{rel}] generated file must end with exactly one newline"
    );
}

fn assert_matches_golden(name: &str, rel: &str, actual: &str) {
    let path = golden_root().join(name).join(rel);

    if blessing() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, actual).unwrap();
        return;
    }

    let expected = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "missing golden file {} — create it with:\n  GLOAM_BLESS=1 cargo test --test golden",
            path.display()
        )
    });

    if expected == actual {
        return;
    }

    // Report the first divergence compactly; a full assert_eq! dump of two
    // multi-hundred-KB strings is unreadable.
    let mut diff_line = None;
    let mut e_lines = expected.lines();
    let mut a_lines = actual.lines();
    let mut i = 0usize;
    loop {
        i += 1;
        match (e_lines.next(), a_lines.next()) {
            (Some(e), Some(a)) if e == a => continue,
            (None, None) => break, // same lines, so difference is in line endings/trailing bytes
            (e, a) => {
                diff_line = Some((i, e.map(str::to_string), a.map(str::to_string)));
                break;
            }
        }
    }
    let detail = match diff_line {
        Some((line, e, a)) => format!(
            "first difference at line {line}:\n  golden: {}\n  actual: {}",
            e.as_deref().unwrap_or("<EOF>"),
            a.as_deref().unwrap_or("<EOF>")
        ),
        None => "contents differ only in line endings or trailing bytes".to_string(),
    };
    panic!(
        "[{name}/{rel}] generated output no longer matches the golden snapshot.\n{detail}\n\n\
         If this change is intended (or follows a bundled-spec update), re-bless and review:\n  \
         GLOAM_BLESS=1 cargo test --test golden\n  git diff tests/golden/"
    );
}

/// Generate one config and snapshot the primary .h/.c pair for `stem`.
fn check(name: &str, global_args: &[&str], c_flags: &[&str], stem: &str) {
    let dir = generate(global_args, c_flags);

    let header = common::read_header(dir.path(), stem);
    let source = common::read_source(dir.path(), stem);

    assert_preprocessor_balance(name, &format!("{stem}.h"), &header);
    assert_preprocessor_balance(name, &format!("{stem}.c"), &source);

    assert_single_trailing_newline(name, &format!("{stem}.h"), &header);
    assert_single_trailing_newline(name, &format!("{stem}.c"), &source);

    assert_matches_golden(name, &format!("{stem}.h"), strip_preamble(&header));
    assert_matches_golden(name, &format!("{stem}.c"), strip_preamble(&source));
}

// ---------------------------------------------------------------------------
// Configs — one per spec family plus the flagship merged build, kept small
// (pinned versions, explicit extension lists) so goldens stay reviewable and
// only churn when resolved content genuinely changes.
// ---------------------------------------------------------------------------

#[test]
fn golden_gl_core_noext() {
    check(
        "gl_core_noext",
        &["--api", "gl:core=3.3", "--extensions", ""],
        &[],
        "gl",
    );
}

#[test]
fn golden_gl_core_full() {
    // --alias + --loader with a couple of extensions: exercises the alias
    // resolver, the loader layer, and extension machinery for desktop GL.
    check(
        "gl_core_full",
        &[
            "--api",
            "gl:core=3.3",
            "--extensions",
            "GL_KHR_debug,GL_ARB_sync",
        ],
        &["--alias", "--loader"],
        "gl",
    );
}

#[test]
fn golden_gles1() {
    // GLES1 exercises the legacy-only (glGetString) extension-query branch;
    // the extension keeps that branch emitted (no extensions, no query).
    check(
        "gles1",
        &[
            "--api",
            "gles1",
            "--extensions",
            "GL_OES_framebuffer_object",
        ],
        &["--loader"],
        "gles1",
    );
}

#[test]
fn golden_merged_gl_gles2() {
    // The flagship production configuration: merged GL+GLES2 with all flags.
    check(
        "merged_gl_gles2",
        &[
            "--api",
            "gl:core=3.3,gles2=3.0",
            "--merge",
            "--extensions",
            "GL_KHR_debug",
        ],
        &["--alias", "--loader"],
        "gl",
    );
}

#[test]
fn golden_egl() {
    check(
        "egl",
        &["--api", "egl", "--extensions", "EGL_KHR_debug"],
        &["--loader"],
        "egl",
    );
}

#[test]
fn golden_glx() {
    check(
        "glx",
        &["--api", "glx", "--extensions", ""],
        &["--loader"],
        "glx",
    );
}

#[test]
fn golden_wgl() {
    // Empty filter still includes the mandatory WGL extensions.
    check(
        "wgl",
        &["--api", "wgl", "--extensions", ""],
        &["--loader"],
        "wgl",
    );
}

#[test]
fn golden_vk_noext() {
    check(
        "vk_noext",
        &["--api", "vk=1.0", "--extensions", ""],
        &["--alias", "--loader"],
        "vk",
    );
}

#[test]
fn golden_vk_ext() {
    // VK_KHR_swapchain pulls in VK_KHR_surface via dependency selection.
    check(
        "vk_ext",
        &["--api", "vk=1.0", "--extensions", "VK_KHR_swapchain"],
        &["--loader"],
        "vk",
    );
}

#[test]
fn golden_vk_external_headers() {
    // External-headers mode: types come from the system Vulkan-Headers, so
    // this exercises the alternate header-template paths and stays small.
    check(
        "vk_external_headers",
        &["--api", "vk=1.3", "--extensions", ""],
        &["--external-headers"],
        "vk",
    );
}
