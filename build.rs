//! Build script: embeds git version information at compile time.
//!
//! Produces `$OUT_DIR/build_info.rs` with constants that `src/build_info.rs`
//! includes.  Every git operation is independently fallible — no `.git`
//! directory (crates.io tarball), shallow clones (no tags), and sparse
//! checkouts are all handled gracefully by falling back to `None`.
//!
//! Scenarios:
//!   cargo build              — .git directory at repo root, full info
//!   cargo install --git=...  — .git *file* in checkout (gitdir: pointer),
//!                              git -C follows it transparently
//!   cargo install gloam      — crates.io tarball, no .git at all,
//!                              all git values are None
//!   shallow clone / CI       — .git exists but tags may be missing,
//!                              GIT_DESCRIBE falls back to None

use std::{env, fs, io, path::Path, path::PathBuf, process::Command};

/// Run a git command in `repo_dir`.  Using `-C` lets git walk up to find
/// `.git` naturally and handles gitdir files (worktrees, cargo git checkouts).
fn git(repo_dir: &Path, args: &[&str]) -> Option<String> {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Check whether a git command exits successfully (ignoring output).
fn git_ok(repo_dir: &Path, args: &[&str]) -> Option<bool> {
    Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .args(args)
        .output()
        .ok()
        .map(|o| o.status.success())
}

/// Find the git repo root for the crate being built.
///
/// Uses `git rev-parse --show-toplevel` from CARGO_MANIFEST_DIR, which
/// lets git itself determine whether we're inside a repo.
///
/// Returns None if:
///   - git isn't installed
///   - we're not in a repo (crates.io tarball)
///   - the detected repo root is an unrelated repo (e.g. the user's home
///     directory under dotfiles management, which contains the cargo
///     registry extraction path)
///
/// The "is this our repo?" check verifies that the repo root contains a
/// Cargo.toml with our package name.  This is more robust than a simple
/// `starts_with` ancestor check, which passes when `cargo install` extracts
/// into `~/.cargo/registry/src/` inside a home-directory git repo.
fn find_repo_root(manifest_dir: &Path) -> Option<PathBuf> {
    let toplevel = git(manifest_dir, &["rev-parse", "--show-toplevel"])?;
    let root = PathBuf::from(&toplevel);

    // The repo root must contain a Cargo.toml that declares our package.
    // This rejects unrelated repos that happen to be ancestors of the
    // extraction directory (e.g. dotfiles repos tracking ~/).
    let cargo_toml = root.join("Cargo.toml");
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    match fs::read_to_string(&cargo_toml) {
        Ok(contents) => {
            // Verify this Cargo.toml declares our package.  Whitespace
            // around `=` varies (some authors align values), so we check
            // that a line contains `name`, `=`, and `"<pkg_name>"` in
            // that order rather than matching a single exact string.
            let quoted_name = format!("\"{}\"", pkg_name);
            let is_ours = contents.lines().any(|line| {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("name") {
                    let rest = rest.trim_start();
                    if let Some(rest) = rest.strip_prefix('=') {
                        return rest.trim().starts_with(&quoted_name);
                    }
                }
                false
            });
            if !is_ours {
                return None;
            }
        }
        Err(_) => return None,
    }

    Some(root)
}

/// Write `contents` to `path`, but only if the file doesn't already exist
/// with identical content.  Avoids triggering unnecessary rebuilds.
fn write_if_changed(path: &Path, contents: &[u8]) -> io::Result<()> {
    if let Ok(existing) = fs::read(path)
        && existing == contents
    {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

fn opt_str(v: &Option<String>) -> String {
    match v {
        Some(s) => format!("Some({:?})", s),
        None => "None".to_string(),
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = find_repo_root(&manifest_dir);

    // Tell cargo when to re-run.  We must resolve the *real* git directory —
    // when .git is a file (worktrees, `cargo install --git`), the actual HEAD,
    // refs, and packed-refs live inside the gitdir target, not next to the .git
    // file.  Cargo treats non-existent rerun-if-changed paths as "always
    // changed", so we must only emit paths that actually exist.
    //
    // IMPORTANT: do NOT watch the `.git` directory itself — cargo watches
    // directories recursively, and git constantly touches files inside `.git/`
    // (index, lock files, FETCH_HEAD, etc.) that have nothing to do with the
    // commit or tag state.  Only watch the specific files that change when
    // the version-relevant state changes.
    //
    // If no repo is found at all, emit just `build.rs` to prevent cargo's
    // default "rerun on any file change" behaviour.
    if let Some(ref root) = repo_root {
        // `git rev-parse --git-dir` follows .git files and returns the real
        // directory (e.g. ~/.cargo/git/db/gloam-xxxx/...).  The result may be
        // relative to the repo root, so resolve it.
        let git_dir = git(root, &["rev-parse", "--git-dir"])
            .map(|p| {
                let path = PathBuf::from(p);
                if path.is_absolute() {
                    path
                } else {
                    root.join(path)
                }
            })
            .unwrap_or_else(|| root.join(".git"));

        // HEAD changes on commit, checkout, rebase, etc.
        // packed-refs changes when refs are packed.
        // refs/heads and refs/tags change on commit/tag operations.
        for name in &["HEAD", "packed-refs", "refs/heads", "refs/tags"] {
            let path = git_dir.join(name);
            if path.exists() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }

        // Also watch .git itself if it's a *file* (gitdir pointer) — if the
        // pointer changes, we need to re-resolve.  But NOT if it's a
        // directory, because that triggers recursive watching.
        let git_entry = root.join(".git");
        if git_entry.is_file() {
            println!("cargo:rerun-if-changed={}", git_entry.display());
        }
    } else {
        // No git — just watch build.rs so we don't re-run on every build.
        println!("cargo:rerun-if-changed=build.rs");
    }

    // Each git query is independent — any can fail without affecting others.
    let (git_describe, git_sha, git_sha_short, git_branch, git_dirty, git_commit_year) =
        match repo_root {
            Some(ref root) => (
                // --tags: match lightweight tags too (not just annotated).
                // --dirty: append "-dirty" if the worktree has uncommitted changes.
                // Deliberately no --always: if no tag is reachable (shallow
                // clone, tags not fetched), we want this to fail so we fall
                // through to the PKG_VERSION+sha format instead of emitting
                // a bare SHA as the version string.
                git(root, &["describe", "--tags", "--dirty"]),
                git(root, &["rev-parse", "HEAD"]),
                git(root, &["rev-parse", "--short", "HEAD"]),
                git(root, &["symbolic-ref", "--short", "-q", "HEAD"]),
                // diff-index exits 0 if clean, 1 if dirty.  Covers tracked files;
                // for untracked files we'd need status --porcelain, but untracked
                // files aren't a meaningful "dirty" signal for version stamping.
                git_ok(root, &["diff-index", "--quiet", "HEAD", "--"]).map(|clean| !clean),
                // Year of HEAD commit — used for copyright years in generated files.
                // %as gives the author date in YYYY-MM-DD format.
                git(root, &["show", "-s", "--format=%as", "HEAD"])
                    .and_then(|s| s.split('-').next().map(str::to_string)),
            ),
            None => (None, None, None, None, None, None),
        };

    // Build year: prefer git commit year (accurate for releases), fall back to
    // the current calendar year (correct for crates.io installs where no git
    // info is available).
    let build_year: u16 = git_commit_year
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            // std::time is available without extra deps.  SystemTime → days → year.
            let secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            // Approximate: 365.25 days/year, close enough for a copyright year.
            (1970 + secs / 31_557_600) as u16
        });

    // Assemble a human-readable version string:
    //   With git describe:  "v0.3.1-7-gabcdef1" or "v0.3.1-7-gabcdef1-dirty"
    //   SHA only (no tags):  "0.3.1+abcdef1" or "0.3.1+abcdef1.dirty"
    //   No git at all:       "0.3.1"
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let version_string = if let Some(ref desc) = git_describe {
        desc.clone()
    } else if let Some(ref sha) = git_sha_short {
        let dirty_suffix = if git_dirty == Some(true) {
            ".dirty"
        } else {
            ""
        };
        format!("{pkg_version}+{sha}{dirty_suffix}")
    } else {
        pkg_version.clone()
    };

    // Expose the target triple to integration tests via env!("TARGET").
    // The cc crate needs this to find the correct compiler.
    println!(
        "cargo:rustc-env=TARGET={}",
        env::var("TARGET").unwrap()
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("build_info.rs");

    // Pre-format values as Rust literals to avoid nested format!() calls.
    let version_string_lit = format!("{:?}", version_string);
    let pkg_version_lit = format!("{:?}", pkg_version);
    let git_describe_lit = opt_str(&git_describe);
    let git_sha_lit = opt_str(&git_sha);
    let git_sha_short_lit = opt_str(&git_sha_short);
    let git_branch_lit = opt_str(&git_branch);
    let git_dirty_lit = match git_dirty {
        Some(b) => format!("Some({b})"),
        None => "None".to_string(),
    };

    let rs = format!(
        r#"// @generated by build.rs — do not edit.

/// Composite version string for display.  Prefers `git describe` output
/// when available, falls back to `CARGO_PKG_VERSION` + short SHA, or just
/// `CARGO_PKG_VERSION` if no git info is available at all.
pub const VERSION: &str = {version_string_lit};

/// Cargo package version from Cargo.toml (always present).
pub const PKG_VERSION: &str = {pkg_version_lit};

/// Full output of `git describe --tags --dirty --always`, if available.
/// Examples: "v0.3.1", "v0.3.1-7-gabcdef1", "v0.3.1-dirty", "abcdef1".
pub const GIT_DESCRIBE: Option<&str> = {git_describe_lit};

/// Full commit SHA, if available.
pub const GIT_SHA: Option<&str> = {git_sha_lit};

/// Abbreviated commit SHA, if available.
pub const GIT_SHA_SHORT: Option<&str> = {git_sha_short_lit};

/// Current branch name, if on a branch (None for detached HEAD).
pub const GIT_BRANCH: Option<&str> = {git_branch_lit};

/// Whether the working tree had uncommitted changes at build time.
pub const GIT_DIRTY: Option<bool> = {git_dirty_lit};

/// Year for copyright notices.  From the git commit date when available,
/// otherwise the calendar year at build time.
pub const BUILD_YEAR: u16 = {build_year};
"#,
    );

    write_if_changed(&dest, rs.as_bytes()).expect("failed to write build_info.rs");
}
