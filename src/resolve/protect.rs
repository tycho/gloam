//! Protection lattice and group-by-protection coalescing.
//!
//! The protection lattice tracks whether a type or include needs platform
//! guards (`#if defined(...)`) or is unconditional.  `group_by_protection`
//! coalesces consecutive items with identical guards into single groups,
//! minimizing `#ifdef`/`#endif` pairs in the generated header.

use std::collections::HashMap;

use crate::ir::{RawExtension, Require};

use super::types::Protect;

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

    /// Convert to the final representation: guards sorted, empty =
    /// unconditional.
    pub fn into_protect(self) -> Protect {
        match self {
            Self::Unconditional => Protect::default(),
            Self::Guarded(v) if v.is_empty() => Protect::default(),
            Self::Guarded(mut v) => {
                v.sort();
                Protect(v)
            }
        }
    }

    /// Test-side assertion helper (production code branches on the variants
    /// directly).
    #[cfg(test)]
    pub fn is_unconditional(&self) -> bool {
        matches!(self, Self::Unconditional)
    }
}

// ---------------------------------------------------------------------------
// Extension-derived item protections
// ---------------------------------------------------------------------------

/// Build a map from item name → protection macros derived from the extensions
/// that require that item.  `items` selects which require-list contributes
/// (`|r| &r.enums` for enum constants, `|r| &r.types` for types).
///
/// This covers the common Vulkan pattern where a struct or constant has no
/// `protect=` attribute of its own but is required only inside an extension
/// with `platform="win32"` (or similar), making its protection implicit.
///
/// Deliberately scoped to ALL extensions in the spec, not just the selected
/// ones: Vulkan emits every flat enum and every auto-category type regardless
/// of selection, so these guards are what keep platform-only and
/// Vulkan-SC-only items behind their `VK_USE_PLATFORM_*` macros — mirroring
/// upstream vulkan_core.h, where every extension's constants are defined and
/// platform items live in guarded platform headers.  Scoping this to selected
/// extensions would strip those guards while the items stay emitted.
pub(super) fn build_ext_protections<F>(
    extensions: &[RawExtension],
    items: F,
) -> HashMap<String, Protect>
where
    F: Fn(&Require) -> &[String],
{
    let mut tmp: HashMap<&str, Protection> = HashMap::new();

    for ext in extensions {
        for require in &ext.requires {
            for name in items(require) {
                tmp.entry(name.as_str())
                    .or_insert_with(Protection::new_guarded)
                    .add_extension(&ext.protect);
            }
        }
    }

    tmp.into_iter()
        .map(|(name, prot)| (name.to_string(), prot.into_protect()))
        .collect()
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
        let v = p.into_protect().0;
        assert_eq!(v.len(), 2);
        // into_protect sorts
        assert_eq!(v[0], "VK_USE_PLATFORM_WIN32_KHR");
        assert_eq!(v[1], "VK_USE_PLATFORM_XLIB_KHR");
    }

    #[test]
    fn protection_guarded_deduplicates() {
        let mut p = Protection::new_guarded();
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        p.add_extension(&["VK_USE_PLATFORM_WIN32_KHR".to_string()]);
        let v = p.into_protect().0;
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn protection_empty_guarded_is_unconditional() {
        // A Guarded with no guards is treated as unconditional.
        let p = Protection::new_guarded();
        assert!(p.into_protect().is_unconditional());
    }

    #[test]
    fn protection_unconditional_converts_to_unconditional() {
        let p = Protection::Unconditional;
        assert!(p.into_protect().is_unconditional());
    }

    // ---- build_ext_protections ----

    fn make_ext(name: &str, protect: &[&str], enums: &[&str], types: &[&str]) -> RawExtension {
        RawExtension {
            name: name.to_string(),
            supported: vec![],
            requires: vec![Require {
                enums: enums.iter().map(|s| s.to_string()).collect(),
                types: types.iter().map(|s| s.to_string()).collect(),
                ..Default::default()
            }],
            protect: protect.iter().map(|s| s.to_string()).collect(),
            depends: vec![],
        }
    }

    #[test]
    fn ext_protections_guarded_by_protected_extension() {
        let exts = vec![make_ext(
            "VK_KHR_win32_surface",
            &["VK_USE_PLATFORM_WIN32_KHR"],
            &["VK_KHR_WIN32_SURFACE_SPEC_VERSION"],
            &["VkWin32SurfaceCreateInfoKHR"],
        )];
        let by_enum = build_ext_protections(&exts, |r| &r.enums);
        let by_type = build_ext_protections(&exts, |r| &r.types);
        assert_eq!(
            by_enum["VK_KHR_WIN32_SURFACE_SPEC_VERSION"].0,
            vec!["VK_USE_PLATFORM_WIN32_KHR"]
        );
        assert_eq!(
            by_type["VkWin32SurfaceCreateInfoKHR"].0,
            vec!["VK_USE_PLATFORM_WIN32_KHR"]
        );
        // The accessor selects the list: enums don't leak into the type map.
        assert!(!by_type.contains_key("VK_KHR_WIN32_SURFACE_SPEC_VERSION"));
    }

    #[test]
    fn ext_protections_unprotected_extension_absorbs_to_unconditional() {
        // An item required by both a protected and an unprotected extension
        // must end up unconditional (empty guard list), regardless of order.
        let exts = vec![
            make_ext(
                "VK_KHR_win32_surface",
                &["VK_USE_PLATFORM_WIN32_KHR"],
                &["VK_SHARED_CONSTANT"],
                &[],
            ),
            make_ext("VK_KHR_surface", &[], &["VK_SHARED_CONSTANT"], &[]),
        ];
        let map = build_ext_protections(&exts, |r| &r.enums);
        assert!(map["VK_SHARED_CONSTANT"].is_unconditional());
    }

    #[test]
    fn ext_protections_guards_union_and_sort_across_extensions() {
        let exts = vec![
            make_ext("ext_b", &["MACRO_B"], &["SHARED"], &[]),
            make_ext("ext_a", &["MACRO_A"], &["SHARED"], &[]),
        ];
        let map = build_ext_protections(&exts, |r| &r.enums);
        assert_eq!(map["SHARED"].0, vec!["MACRO_A", "MACRO_B"]);
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
