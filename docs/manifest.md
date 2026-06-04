# gloam manifest & provenance — consumer guide

> **Status:** design spec, not yet implemented. This document describes the
> intended format and behavior. See [provenance-internals.md](provenance-internals.md)
> for the producer-side design.

Every time gloam generates a loader it records exactly what went into the
output: which gloam version, which command line, and which upstream source
files (by repository, commit, `git describe`, and content hash). This guide is
for anyone who wants to **read** that information — to audit a checked-in
loader, verify integrity, or reproduce a generated tree.

There are two places this information appears:

1. A machine-readable manifest written to `.gloam/manifest.json` at the root of
   the output tree.
2. A human-readable block in the comment header of every generated `.h`/`.c`
   file.

---

## 1. `.gloam/manifest.json`

A single pretty-printed JSON file describing the whole output tree. It is
pretty-printed (not minified) on purpose, so line-based diffs stay legible when
the file is checked into version control.

```jsonc
{
  "schema_version": 1,

  // --- gloam self-metadata: who generated this and how -------------------
  "gloam": {
    "version": "0.4.9",                       // crate version
    "describe": "v0.4.9-3-g8498f7e",          // git describe of the gloam build
    "commit": "8498f7ec…",                    // full gloam commit SHA-1
    "command_line": "gloam --api gl:core=3.3 c --loader"
  },

  // --- source provenance: the pin set ------------------------------------
  // Keyed by the file's logical name (the same name used in `derived_from`
  // and in the generated headers). This is the immutable identity of every
  // upstream input gloam knows about for this tree. It is preserved verbatim
  // across regenerations (see "Reproducing output" below): it may contain
  // entries that no current output file references.
  "provenance": {
    "xml/gl.xml": {
      "repo": "KhronosGroup/OpenGL-Registry",
      "repo_url": "https://github.com/KhronosGroup/OpenGL-Registry",
      "path_in_repo": "xml/gl.xml",
      "commit": "a1b2c3d4…",                  // full upstream commit SHA-1
      "describe": "a1b2c3d",                  // git describe (bare commit if untagged)
      "blob": "0fa1e2d3…"                     // git blob SHA-1 of the file content
    },
    "xxhash.h": {
      "repo": "Cyan4973/xxHash",
      "repo_url": "https://github.com/Cyan4973/xxHash",
      "path_in_repo": "xxhash.h",
      "commit": "7e3f2a19…",
      "describe": "v0.8.2-9-g7e3f2a1",
      "blob": "abcabc1…"
    }
    // … one entry per upstream input
  },

  // --- output BOM: what was produced, and from what ----------------------
  // One entry per file in the output tree. `derived_from` lists the
  // provenance keys that influenced this specific file. `verbatim: true`
  // marks files copied byte-for-byte from upstream (auxiliary headers).
  "output": [
    {
      "path": "include/gloam/gl.h",
      "blob": "…",                            // git blob SHA-1 of the output file
      "derived_from": ["xml/gl.xml", "KHR/khrplatform.h", "xxhash.h"]
    },
    {
      "path": "include/xxhash.h",
      "blob": "…",
      "verbatim": true,
      "derived_from": ["xxhash.h"]
    }
    // …
  ]
}
```

### Field reference

**`gloam`** — provenance of the generator itself.

| field | meaning |
| --- | --- |
| `version` | gloam crate version |
| `describe` | `git describe`-style version of the gloam build |
| `commit` | full gloam commit SHA-1 |
| `command_line` | the exact invocation that reproduces this tree (with the same gloam version and the same `provenance`) |

**`provenance`** — the pin set: an object keyed by logical file name. Each value
identifies one upstream file by content.

| field | meaning |
| --- | --- |
| `repo` | `owner/name` slug of the upstream repository |
| `repo_url` | clonable/browsable URL |
| `path_in_repo` | path of the file within that repository |
| `commit` | full upstream commit SHA-1 the file was taken from |
| `describe` | `git describe` of that commit (a bare short commit when the repo has no reachable tags) |
| `blob` | git blob SHA-1 of the file content — a stable, verifiable content hash |

**`output`** — the bill of materials.

| field | meaning |
| --- | --- |
| `path` | output path, relative to the tree root |
| `blob` | git blob SHA-1 of the generated file (equals `git hash-object <file>`) |
| `verbatim` | present and `true` when the file is an upstream file copied unchanged |
| `derived_from` | list of `provenance` keys that influenced this file |

### Determinism guarantee

The manifest contains **no timestamps** and nothing else that varies with *when*
generation happened. Given the same gloam version, the same command line, and
the same upstream content (same commits/blobs), the manifest and every generated
file are **byte-identical** — regardless of when or where they were produced.

This is deliberate: loaders are checked into downstream projects and their diffs
are audited. A regeneration that pulls the same upstream content produces no
diff at all.

### Verifying integrity

Every hash is a git blob SHA-1, so you can verify any file with stock git:

```sh
git hash-object include/gloam/gl.h     # must equal the output entry's "blob"
```

Because the upstream `blob` fields are also git blob SHA-1s, a file fetched from
GitHub by blob id (`GET /repos/{owner}/{repo}/git/blobs/{sha}`) is guaranteed to
be the exact content recorded here.

---

## 2. The generated-source header block

Each generated file carries the same provenance, formatted for humans. The
**sources block is scoped to that specific file** — it lists only the inputs
that influenced *this* file, never the full command line's inputs. For a merged
`gl:core,gles2,egl` build, the GL loader's header names only GL-family sources;
the EGL loader's header names only EGL-family sources.

```c
/*
 * Generated by gloam v0.4.9-3-g8498f7e.
 *
 *   gloam --api gl:core=3.3 c --loader
 *
 * Extensions: 5 explicit, 12 promoted (17 included)
 *
 * Copyright (c) 2026 Steven Noonan
 * SPDX-License-Identifier: MIT
 *
 * Portions derived from Khronos Group XML API Registry specifications.
 * Copyright (c) 2013-2026 The Khronos Group Inc.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Portions derived from xxHash.
 * Copyright (c) 2012-2026 Yann Collet.
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * Generated from the following upstream sources:
 *
 *   Cyan4973/xxHash (v0.8.2-9-g7e3f2a1)
 *     xxhash.h (blob abcabc1)
 *   KhronosGroup/EGL-Registry (8e3c4f1)
 *     api/KHR/khrplatform.h (blob 5a5a5a5)
 *   KhronosGroup/OpenGL-Registry (a1b2c3d)
 *     xml/gl.xml (blob 0fa1e2d)
 *   tycho/gloam (v0.4.9-3-g8498f7e)
 *     bundled/xml/glsl_exts.xml (blob 1212121)
 *
 * DO NOT EDIT. This file is generated by gloam and will be overwritten;
 * make changes to the gloam invocation or inputs, not to this file.
 */
```

Reading the block top to bottom:

- **Generated by …** — the gloam version, then the reproducing command line.
- **Extensions: …** — a summary of how the extension set was selected.
- **Copyright notices** — gloam's own (MIT) first, then one notice per distinct
  copyright holder/license among the *contributing* sources. Holders are not
  repeated: all Khronos registries collapse into a single Apache-2.0 notice. A
  notice only appears if a file under that license actually influenced this
  file (e.g. the xxHash notice appears only in loaders that emit `xxhash.h`).
- **Generated from the following upstream sources** — grouped by repository.
  The repository line carries the `git describe`; each file under it carries its
  blob hash. This two-level layout keeps a repo-wide commit bump from churning
  every file's line in a diff.
- **DO NOT EDIT** — a footer; edits are overwritten on the next generation.

---

## 3. Reproducing output with `--lock`

You can feed a manifest back into gloam to pin the upstream sources, then
regenerate with the same or different flags:

```sh
gloam --lock path/to/.gloam/manifest.json --api gl:core=4.6 c --loader
```

When reading a `--lock` manifest, gloam uses **only its `provenance` section**.
The `gloam` and `output` sections of the input are ignored and regenerated
fresh for the new run.

**You must also make the locked content available**, one of two ways:

1. pass `--fetch` (gloam fetches each pinned blob by id, cache-first), or
2. use a gloam build whose **bundled files match the locked blobs**.

gloam compares each required file's pinned `blob` against its bundled copy. If
they all match, `--lock` works offline against bundled content; if any differ or
are missing, gloam refuses and asks you to add `--fetch`, because the bundled
content can't satisfy that lock.

Semantics:

- **Exact bytes.** Every required source is fetched by its pinned `blob` id, so
  you get the exact upstream content recorded in the manifest, regardless of
  what has since changed upstream. With the same gloam version, output is
  byte-identical to the original for the overlapping files.
- **Insufficient provenance ⇒ refusal.** gloam computes the set of source files
  the new command line requires. If any required file is missing from the
  manifest's `provenance`, gloam refuses and tells you which file is missing.
  Example: a manifest captured from `gl:core` alone has no `gl_angle_ext.xml`
  pin, so reusing it to generate `gl:core,gles2` fails — regenerate without
  `--lock` to capture fresh provenance.
- **Verbatim carry-forward.** The output manifest preserves the input's
  `provenance` section **unchanged** — nothing is added or removed, even if the
  new run uses only a subset. This lets you trim a broad loader to one API and
  later expand it again from the same lock. Pins not referenced by any current
  output file simply remain unreferenced; that is expected.

### Snapshot manifests

gloam can also emit a **manifest-only snapshot** covering every supported API
and every possible input file, without generating any loader. This gives you a
single point-in-time lock you can reuse across many different loader
generations. See the producer guide for the subcommand.

---

## Schema versioning

`schema_version` is bumped when the manifest layout changes incompatibly.
Consumers should check it and reject versions they do not understand. Additive,
backward-compatible fields may be introduced without a bump.
