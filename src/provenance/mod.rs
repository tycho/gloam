//! Upstream source provenance: the static registry of where every bundled /
//! fetchable file comes from, the attribution table, and the runtime types that
//! carry resolved provenance (commit / describe / blob) through generation.
//!
//! The registry groups files into **repository clusters**: provenance (commit,
//! `git describe`, tree) is resolved once per cluster and shared by every file
//! from it.  This is both correct (a snapshot shares one commit) and cheap.
//!
//! These types are available in every build.  Only acquisition (network) and
//! the cache (SQLite) are gated behind the `fetch` feature.

/// Serializable manifest / bundle-provenance schemas (all builds).
pub mod manifest;

/// Unified source loading (pins + content) for bundled and fetch modes.
pub mod load;

/// GitHub-API provenance acquisition (network).
#[cfg(feature = "fetch")]
pub mod acquire;

/// Local SQLite provenance cache.
#[cfg(feature = "fetch")]
pub mod cache;

/// Unified fetch engine (acquisition + cache).
#[cfg(feature = "fetch")]
pub mod engine;

/// Mock-server tests for the acquisition + cache + engine path (no real API).
#[cfg(all(test, feature = "fetch"))]
mod fetch_tests;

// ---------------------------------------------------------------------------
// Attribution
// ---------------------------------------------------------------------------

/// A copyright/license attribution, shared by all clusters from the same
/// rights holder so notices collapse (e.g. every Khronos repo → one notice).
#[derive(Debug)]
pub struct Attribution {
    /// Stable id (for grouping/debugging), e.g. "khronos".
    pub id: &'static str,
    /// The "Portions derived from …" sentence for the copyright block.
    /// Empty for `is_self` attributions (gloam's own files), which are covered
    /// by gloam's primary MIT notice and get no separate line.
    pub blurb: &'static str,
    /// Copyright holder, e.g. "The Khronos Group Inc.".
    pub holder: &'static str,
    /// SPDX license identifier, e.g. "Apache-2.0".
    pub license: &'static str,
    /// First copyright year (range end is the deterministic build year).
    pub year_start: u16,
    /// True for gloam's own files (glsl_exts.xml): no separate copyright line.
    pub is_self: bool,
}

pub static ATTR_KHRONOS: Attribution = Attribution {
    id: "khronos",
    blurb: "Portions derived from Khronos Group XML API Registry specifications.",
    holder: "The Khronos Group Inc.",
    license: "Apache-2.0",
    year_start: 2013,
    is_self: false,
};

pub static ATTR_ANGLE: Attribution = Attribution {
    id: "angle",
    blurb: "Includes extensions from the ANGLE project.",
    holder: "The ANGLE Project Authors",
    license: "BSD-3-Clause",
    year_start: 2018,
    is_self: false,
};

pub static ATTR_XXHASH: Attribution = Attribution {
    id: "xxhash",
    blurb: "Portions derived from xxHash.",
    holder: "Yann Collet",
    license: "BSD-2-Clause",
    year_start: 2012,
    is_self: false,
};

pub static ATTR_GLOAM: Attribution = Attribution {
    id: "gloam",
    blurb: "",
    holder: "Steven Noonan",
    license: "MIT",
    year_start: 2024,
    is_self: true,
};

// ---------------------------------------------------------------------------
// Static registry
// ---------------------------------------------------------------------------

/// One file within a cluster.
#[derive(Debug)]
pub struct FileSpec {
    /// Logical key — unique across the whole registry.  Used as the manifest
    /// provenance key, the `derived_from` cross-reference, and (for auxiliary
    /// headers) the output include path.  For XML specs this is the filename.
    pub key: &'static str,
    /// Path of the file within its repository, e.g. "xml/gl.xml".
    pub path_in_repo: &'static str,
}

/// A repository cluster: one upstream repo at one branch.
#[derive(Debug)]
pub struct Cluster {
    /// "owner/name" slug, e.g. "KhronosGroup/OpenGL-Registry".
    pub repo: &'static str,
    /// Browsable/clonable repository URL.
    pub repo_url: &'static str,
    /// Branch tracked for HEAD resolution.
    pub branch: &'static str,
    /// Attribution shared by every file in this cluster.
    pub attribution: &'static Attribution,
    /// Files we may take from this cluster.
    pub files: &'static [FileSpec],
}

pub static CLUSTERS: &[Cluster] = &[
    Cluster {
        repo: "KhronosGroup/OpenGL-Registry",
        repo_url: "https://github.com/KhronosGroup/OpenGL-Registry",
        branch: "main",
        attribution: &ATTR_KHRONOS,
        files: &[
            FileSpec {
                key: "gl.xml",
                path_in_repo: "xml/gl.xml",
            },
            FileSpec {
                key: "glx.xml",
                path_in_repo: "xml/glx.xml",
            },
            FileSpec {
                key: "wgl.xml",
                path_in_repo: "xml/wgl.xml",
            },
        ],
    },
    Cluster {
        repo: "KhronosGroup/EGL-Registry",
        repo_url: "https://github.com/KhronosGroup/EGL-Registry",
        branch: "main",
        attribution: &ATTR_KHRONOS,
        files: &[
            FileSpec {
                key: "egl.xml",
                path_in_repo: "api/egl.xml",
            },
            FileSpec {
                key: "KHR/khrplatform.h",
                path_in_repo: "api/KHR/khrplatform.h",
            },
            FileSpec {
                key: "EGL/eglplatform.h",
                path_in_repo: "api/EGL/eglplatform.h",
            },
        ],
    },
    Cluster {
        repo: "KhronosGroup/Vulkan-Docs",
        repo_url: "https://github.com/KhronosGroup/Vulkan-Docs",
        branch: "main",
        attribution: &ATTR_KHRONOS,
        files: &[FileSpec {
            key: "vk.xml",
            path_in_repo: "xml/vk.xml",
        }],
    },
    Cluster {
        repo: "KhronosGroup/Vulkan-Headers",
        repo_url: "https://github.com/KhronosGroup/Vulkan-Headers",
        branch: "main",
        attribution: &ATTR_KHRONOS,
        files: &[
            FileSpec {
                key: "vulkan/vk_platform.h",
                path_in_repo: "include/vulkan/vk_platform.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codecs_common.h",
                path_in_repo: "include/vk_video/vulkan_video_codecs_common.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h264std.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h264std.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h264std_decode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h264std_decode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h264std_encode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h264std_encode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h265std.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h265std.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h265std_decode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h265std_decode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_h265std_encode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_h265std_encode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_av1std.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_av1std.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_av1std_decode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_av1std_decode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_av1std_encode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_av1std_encode.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_vp9std.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_vp9std.h",
            },
            FileSpec {
                key: "vk_video/vulkan_video_codec_vp9std_decode.h",
                path_in_repo: "include/vk_video/vulkan_video_codec_vp9std_decode.h",
            },
        ],
    },
    Cluster {
        repo: "google/angle",
        repo_url: "https://github.com/google/angle",
        branch: "main",
        attribution: &ATTR_ANGLE,
        files: &[
            FileSpec {
                key: "gl_angle_ext.xml",
                path_in_repo: "scripts/gl_angle_ext.xml",
            },
            FileSpec {
                key: "egl_angle_ext.xml",
                path_in_repo: "scripts/egl_angle_ext.xml",
            },
        ],
    },
    Cluster {
        repo: "Cyan4973/xxHash",
        repo_url: "https://github.com/Cyan4973/xxHash",
        branch: "dev",
        attribution: &ATTR_XXHASH,
        files: &[FileSpec {
            key: "xxhash.h",
            path_in_repo: "xxhash.h",
        }],
    },
    Cluster {
        repo: "tycho/gloam",
        repo_url: "https://github.com/tycho/gloam",
        branch: "master",
        attribution: &ATTR_GLOAM,
        files: &[FileSpec {
            key: "glsl_exts.xml",
            path_in_repo: "bundled/xml/glsl_exts.xml",
        }],
    },
];

// ---------------------------------------------------------------------------
// Registry lookups
// ---------------------------------------------------------------------------

/// Every registry file key, in registry order.
pub fn all_keys() -> Vec<&'static str> {
    CLUSTERS
        .iter()
        .flat_map(|c| c.files.iter().map(|f| f.key))
        .collect()
}

/// Find the cluster and file spec for a logical key.
pub fn find(key: &str) -> Option<(&'static Cluster, &'static FileSpec)> {
    for cluster in CLUSTERS {
        for file in cluster.files {
            if file.key == key {
                return Some((cluster, file));
            }
        }
    }
    None
}

/// Relative path, under the repository's `bundled/` directory, where a registry
/// file's bytes are stored on disk and embedded.  XML specs live under `xml/`,
/// everything else under `headers/` (preserving any subdirectory in the key).
pub fn bundled_rel_path(key: &str) -> String {
    if key.ends_with(".xml") {
        format!("xml/{key}")
    } else {
        format!("headers/{key}")
    }
}

/// The primary spec XML key for a spec family name (`gl`, `egl`, …).
pub fn primary_key(spec_name: &str) -> Option<&'static str> {
    Some(match spec_name {
        "gl" => "gl.xml",
        "egl" => "egl.xml",
        "glx" => "glx.xml",
        "wgl" => "wgl.xml",
        "vk" => "vk.xml",
        _ => return None,
    })
}

/// Supplemental XML keys merged into a spec, given the canonical API names in
/// scope for this generation.  The merge is **request-aware**: ANGLE's GLES
/// extension XML is merged only when a GLES API is requested, so a desktop-only
/// `gl:core` loader neither parses nor attributes it.
///
/// - `gl`: `glsl_exts.xml` always (it carries `supported="gl"` and
///   `supported="gl|gles2"` entries); `gl_angle_ext.xml` only when `gles1`/
///   `gles2` is in scope.
/// - `egl`: `egl_angle_ext.xml` (the EGL spec is only loaded when EGL is
///   requested).
pub fn supplemental_keys(spec_name: &str, apis: &[&str]) -> Vec<&'static str> {
    let mut out = Vec::new();
    match spec_name {
        "gl" => {
            out.push("glsl_exts.xml");
            if apis.iter().any(|a| matches!(*a, "gles1" | "gles2")) {
                out.push("gl_angle_ext.xml");
            }
        }
        "egl" => out.push("egl_angle_ext.xml"),
        _ => {}
    }
    out
}

// ---------------------------------------------------------------------------
// Source grouping (shared by the header preamble and --version)
// ---------------------------------------------------------------------------

/// One repository's contribution: its pinned commit and the files taken from
/// it (path + blob), for the grouped "upstream sources" listing.
pub struct RepoSources {
    pub repo: String,
    /// Full commit SHA-1 (displays shorten it).
    pub commit: String,
    /// (path_in_repo, blob), sorted by path.
    pub files: Vec<(String, String)>,
}

/// Group provenance pins by repository — repos sorted case-insensitively by
/// slug, files within a repo sorted by path.  Deterministic; shared by the
/// generated-header sources block and `--version` so the two never drift.
pub fn group_pins_by_repo(
    pins: &indexmap::IndexMap<String, manifest::ProvenancePin>,
) -> Vec<RepoSources> {
    let mut by_repo: indexmap::IndexMap<String, RepoSources> = indexmap::IndexMap::new();
    for pin in pins.values() {
        let group = by_repo
            .entry(pin.repo.clone())
            .or_insert_with(|| RepoSources {
                repo: pin.repo.clone(),
                commit: pin.commit.clone(),
                files: Vec::new(),
            });
        group
            .files
            .push((pin.path_in_repo.clone(), pin.blob.clone()));
    }
    let mut groups: Vec<RepoSources> = by_repo.into_values().collect();
    groups.sort_by_key(|g| g.repo.to_lowercase());
    for g in &mut groups {
        g.files.sort();
    }
    groups
}

// ---------------------------------------------------------------------------
// Runtime resolved provenance
// ---------------------------------------------------------------------------

/// Resolved provenance for one repository cluster at a specific snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedRepo {
    pub repo: String,
    pub repo_url: String,
    pub branch: String,
    /// Full upstream commit SHA-1.
    pub commit: String,
}

/// Resolved provenance for one file: its repo snapshot plus the file's blob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedFile {
    /// Registry key.
    pub key: String,
    /// Path within the repository.
    pub path_in_repo: String,
    /// Git blob SHA-1 of the content (content hash; verifiable with git).
    pub blob: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn keys_are_unique_across_registry() {
        let mut seen = HashSet::new();
        for cluster in CLUSTERS {
            for file in cluster.files {
                assert!(
                    seen.insert(file.key),
                    "duplicate registry key: {}",
                    file.key
                );
            }
        }
    }

    #[test]
    fn find_resolves_cluster_and_file() {
        let (cluster, file) = find("gl.xml").expect("gl.xml in registry");
        assert_eq!(cluster.repo, "KhronosGroup/OpenGL-Registry");
        assert_eq!(file.path_in_repo, "xml/gl.xml");

        let (cluster, file) = find("xxhash.h").expect("xxhash.h in registry");
        assert_eq!(cluster.repo, "Cyan4973/xxHash");
        assert_eq!(cluster.branch, "dev");
        assert_eq!(file.path_in_repo, "xxhash.h");

        assert!(find("does-not-exist.xml").is_none());
    }

    #[test]
    fn primary_keys_map_to_registry_entries() {
        for spec in ["gl", "egl", "glx", "wgl", "vk"] {
            let key = primary_key(spec).unwrap_or_else(|| panic!("no primary for {spec}"));
            assert!(find(key).is_some(), "primary key {key} not in registry");
        }
        assert!(primary_key("nope").is_none());
    }

    #[test]
    fn angle_and_egl_files_live_in_distinct_clusters() {
        // Regression guard for the misattribution bug: a GL-only loader must be
        // able to exclude egl_angle_ext.xml, which requires the two ANGLE files
        // to be individually addressable (they share a cluster but distinct
        // keys).
        let (gl_c, _) = find("gl_angle_ext.xml").unwrap();
        let (egl_c, _) = find("egl_angle_ext.xml").unwrap();
        assert_eq!(gl_c.repo, "google/angle");
        assert_eq!(egl_c.repo, "google/angle");
    }
}
