//! Diagnostics reporting.
//!
//! One handle threaded through the pipeline so every informational and
//! warning message goes through a single policy point.  Per CONTRIBUTING.md,
//! progress output and warnings go to stderr and respect `--quiet`; only
//! hard errors (surfaced as `anyhow` errors) are unconditional.
//!
//! # Malformed-input policy
//!
//! The Khronos XML is hand-maintained and occasionally damaged.  gloam
//! responds by severity, not by which parser happened to hit the problem:
//!
//! - **Error** (`anyhow::bail!`): structural damage to the API skeleton —
//!   a `<feature>` missing its name/api/number or carrying an unparseable
//!   version, an `<enums>` entry without a name, a conflicting duplicate
//!   enum value.  Silently dropping these would produce a loader that
//!   quietly lacks an API level or constant.  Selected-but-missing commands
//!   are likewise a hard error at resolve time.
//! - **Warn** ([`Diag::warn`], suppressed by `--quiet`): damage in content
//!   that may never be selected — an unnameable `<type>` or alias-only
//!   `<command>`, an unresolvable command alias chain, a missing mandatory
//!   WGL extension.  Parsing continues; if the damaged item ends up
//!   mattering, resolution fails loudly instead.
//! - **Silent**: expected, documented spec quirks (e.g. the broken GLX
//!   extensions of spec gotcha #8), which are handled exactly as their
//!   gotcha comment describes.

/// Process-level mirror of the `--quiet` flag, recorded by [`Diag::new`] so
/// that code without a `Diag` handle (the engine's endpoint-failover path)
/// can still honor it via the free [`warn`].
static QUIET: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
pub struct Diag {
    quiet: bool,
}

impl Diag {
    pub fn new(quiet: bool) -> Self {
        QUIET.store(quiet, std::sync::atomic::Ordering::Relaxed);
        Self { quiet }
    }

    /// A condition worth telling the user about that doesn't invalidate the
    /// output.  Suppressed by `--quiet`.
    pub fn warn(&self, msg: impl std::fmt::Display) {
        if !self.quiet {
            eprintln!("gloam: warning: {msg}");
        }
    }

    /// Progress/status line.  Suppressed by `--quiet`.
    pub fn info(&self, msg: impl std::fmt::Display) {
        if !self.quiet {
            eprintln!("gloam: {msg}");
        }
    }
}

/// Warning from provenance/fetch code that doesn't carry a [`Diag`] handle
/// (the engine's endpoint-failover path).  Honors `--quiet` via the
/// process-level flag recorded by [`Diag::new`] — per CONTRIBUTING.md, only
/// errors print unconditionally.  A suppressed failover is still visible in
/// the `GLOAM_DEBUG` HTTP trace (the failed request is logged there).
#[cfg(feature = "fetch")]
pub fn warn(msg: impl std::fmt::Display) {
    if !QUIET.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("gloam: warning: {msg}");
    }
}

/// True when the `GLOAM_DEBUG` environment variable is set (non-empty).
pub fn debug_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("GLOAM_DEBUG").is_some_and(|v| !v.is_empty()))
}

/// Tracing line for `GLOAM_DEBUG=1` runs: network requests, cache/engine
/// activity, and per-phase timings.  Deliberately independent of `--quiet`
/// (debugging a --quiet invocation must still trace) and of the `Diag`
/// handle (callers deep in the provenance stack don't carry one).
pub fn debug(msg: impl std::fmt::Display) {
    if debug_enabled() {
        eprintln!("gloam: debug: {msg}");
    }
}
