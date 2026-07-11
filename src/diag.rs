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

#[derive(Debug, Clone, Copy)]
pub struct Diag {
    quiet: bool,
}

impl Diag {
    pub fn new(quiet: bool) -> Self {
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
