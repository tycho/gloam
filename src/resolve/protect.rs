//! Protection lattice and group-by-protection coalescing.
//!
//! The protection lattice tracks whether a type or include needs platform
//! guards (`#if defined(...)`) or is unconditional.  `group_by_protection`
//! coalesces consecutive items with identical guards into single groups,
//! minimizing `#ifdef`/`#endif` pairs in the generated header.

use serde::Serialize;

use super::types::ProtectedGroup;

// ---------------------------------------------------------------------------
// Protection lattice
// ---------------------------------------------------------------------------

/// Tracks the platform-protection state for a type or include.
///
/// The lattice has two states:
///   - `Guarded(macros)` — protected by a set of `#if defined(...)` macros.
///   - `Unconditional` — required by at least one unprotected context, so no
///     guard is needed.
///
/// Once a value reaches `Unconditional` it stays there (absorbing element).
#[derive(Debug, Clone)]
pub(super) enum Protection {
    Unconditional,
    Guarded(Vec<String>),
}

impl Protection {
    pub fn new_guarded() -> Self {
        Self::Guarded(Vec::new())
    }

    /// Merge protection information from one extension.  If the extension is
    /// unprotected, the result becomes unconditional.  Otherwise the
    /// extension's guards are unioned in.
    pub fn add_extension(&mut self, ext_protect: &[String]) {
        match self {
            Self::Unconditional => {} // absorbing — nothing to do
            Self::Guarded(guards) => {
                if ext_protect.is_empty() {
                    *self = Self::Unconditional;
                } else {
                    for p in ext_protect {
                        if !guards.contains(p) {
                            guards.push(p.clone());
                        }
                    }
                }
            }
        }
    }

    /// Convert to the final representation: empty Vec = unconditional.
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::Unconditional => Vec::new(),
            Self::Guarded(v) if v.is_empty() => Vec::new(),
            Self::Guarded(mut v) => {
                v.sort();
                v
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_unconditional(&self) -> bool {
        matches!(self, Self::Unconditional)
    }
}

// ---------------------------------------------------------------------------
// Group-by-protection coalescing
// ---------------------------------------------------------------------------

/// Coalesce items into groups of consecutive items that share the same
/// protection macro set.  A single linear pass — O(n) in the item count.
///
/// Takes `(Vec<String>, T)` pairs where the first element is the protection
/// list for that item.  This signature avoids a closure parameter and
/// eliminates the duplicated manual grouping loops that existed for
/// ext_guard_groups and cmd_pfn_groups.
pub(super) fn group_by_protection_pairs<T>(
    items: impl IntoIterator<Item = (Vec<String>, T)>,
) -> Vec<ProtectedGroup<T>>
where
    T: std::fmt::Debug + Serialize,
{
    let mut groups: Vec<ProtectedGroup<T>> = Vec::new();
    for (protect, item) in items {
        if let Some(last) = groups.last_mut()
            && last.protect == protect
        {
            last.items.push(item);
            continue;
        }
        groups.push(ProtectedGroup {
            protect,
            items: vec![item],
        });
    }
    groups
}

/// Convenience wrapper: coalesce items using a closure to extract protection.
pub(super) fn group_by_protection<T, F>(
    items: impl IntoIterator<Item = T>,
    get_protect: F,
) -> Vec<ProtectedGroup<T>>
where
    T: std::fmt::Debug + Serialize,
    F: Fn(&T) -> Vec<String>,
{
    let mut groups: Vec<ProtectedGroup<T>> = Vec::new();
    for item in items {
        let protect = get_protect(&item);
        if let Some(last) = groups.last_mut()
            && last.protect == protect
        {
            last.items.push(item);
            continue;
        }
        groups.push(ProtectedGroup {
            protect,
            items: vec![item],
        });
    }
    groups
}

// ---------------------------------------------------------------------------
// GL auto-exclude set
// ---------------------------------------------------------------------------

/// Type names that GL specs auto-include but that we never emit (they map to
/// system or bundled headers instead).
const GL_AUTO_EXCLUDE: &[&str] = &["stddef", "khrplatform", "inttypes"];

pub(super) fn is_gl_auto_excluded(name: &str) -> bool {
    GL_AUTO_EXCLUDE.contains(&name)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Protection lattice ----

    #[test]
    fn protection_guarded_absorbs_unconditional() {
        // A guarded state that encounters an unprotected extension becomes
        // unconditional (absorbing element of the lattice).
        let mut p = Protection::Guarded(vec!["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&[]); // unprotected extension
        assert!(p.is_unconditional());
    }

    #[test]
    fn protection_unconditional_stays_unconditional() {
        // Once unconditional, adding a guarded extension can't re-guard it.
        let mut p = Protection::Unconditional;
        p.add_extension(&["VK_USE_PLATFORM_XLIB_KHR".to_string()]);
        assert!(p.is_unconditional());
    }

    #[test]
    fn protection_guarded_unions_guards() {
        let mut p = Protection::new_guarded();
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&["VK_USE_PLATFORM_XLIB_KHR".to_string()]);
        let v = p.into_vec();
        assert_eq!(v.len(), 2);
        // into_vec sorts
        assert_eq!(v[0], "VK_USE_PLATFORM_WIN32_KHR");
        assert_eq!(v[1], "VK_USE_PLATFORM_XLIB_KHR");
    }

    #[test]
    fn protection_guarded_deduplicates() {
        let mut p = Protection::new_guarded();
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        let v = p.into_vec();
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn protection_empty_guarded_into_vec_is_empty() {
        // A Guarded with no guards is treated as unconditional (empty Vec).
        let p = Protection::new_guarded();
        assert!(p.into_vec().is_empty());
    }

    #[test]
    fn protection_unconditional_into_vec_is_empty() {
        let p = Protection::Unconditional;
        assert!(p.into_vec().is_empty());
    }

    // ---- is_gl_auto_excluded ----

    #[test]
    fn gl_auto_excluded_known_names() {
        assert!(is_gl_auto_excluded("stddef"));
        assert!(is_gl_auto_excluded("khrplatform"));
        assert!(is_gl_auto_excluded("inttypes"));
    }

    #[test]
    fn gl_auto_excluded_other_names() {
        assert!(!is_gl_auto_excluded("GLuint"));
        assert!(!is_gl_auto_excluded("GLenum"));
    }
}
