# Provenance & manifest — producer internals

> **Status:** design spec, not yet implemented. This is the maintainer-facing
> companion to [manifest.md](manifest.md) (the consumer guide). It records the
> design decisions and invariants for *producing* provenance, the cache, the
> embedded bundle, and the generated headers.

Audience: gloam maintainers. If you only want to read a manifest, see
[manifest.md](manifest.md).

---

## Goals

1. Record the exact upstream provenance of every input that influences a
   generated loader: repository, commit, `git describe`, and content (blob)
   hash.
2. Surface it in `--version`, in the per-file generated header block, and in
   `.gloam/manifest.json`.
3. Make committed output **byte-identical** for identical inputs + gloam
   version, with **no timestamps** anywhere in committed artifacts.
4. Scope provenance to what actually influenced each **output file** — not to
   the whole command line. This is what fixes the long-standing ANGLE
   misattribution bug (see below).
5. Use one acquisition path for `--fetch`, the embedded bundle, and `--lock`.

---

## Acquisition model: per-repo clusters, git-object walking

Upstream files are grouped into **repository clusters**. Provenance is resolved
**once per cluster** and shared across every file from that cluster — this is
both correct (all files in a snapshot share one commit) and cheap (it collapses
~25 files into a handful of clusters).

| cluster (repo) | branch | files | license / holder |
| --- | --- | --- | --- |
| `KhronosGroup/OpenGL-Registry` | `main` | `xml/gl.xml`, `xml/glx.xml`, `xml/wgl.xml` | Apache-2.0 / The Khronos Group Inc. |
| `KhronosGroup/EGL-Registry` | `main` | `api/egl.xml`, `api/KHR/khrplatform.h`, `api/EGL/eglplatform.h` | Apache-2.0 / The Khronos Group Inc. |
| `KhronosGroup/Vulkan-Docs` | `main` | `xml/vk.xml` | Apache-2.0 / The Khronos Group Inc. |
| `KhronosGroup/Vulkan-Headers` | `main` | `include/vulkan/vk_platform.h`, `include/vk_video/*` | Apache-2.0 / The Khronos Group Inc. |
| `google/angle` | `main` | `scripts/gl_angle_ext.xml`, `scripts/egl_angle_ext.xml` | BSD-3-Clause / The ANGLE Project Authors |
| `Cyan4973/xxHash` | `dev` | `xxhash.h` | BSD-2-Clause / Yann Collet |
| `tycho/gloam` | `master` | `bundled/xml/glsl_exts.xml` | MIT / Steven Noonan |

### Resolving a cluster (race-free)

For each cluster we resolve a consistent snapshot pinned to one commit:

1. `GET /repos/{o}/{r}/git/ref/heads/{branch}` → tip **commit** SHA.
2. Derive a **`git describe` equivalent**: enumerate tags
   (`GET /repos/{o}/{r}/tags`, paginated, capped) into a tagged-commit map, then
   scan commits from HEAD (`GET /repos/{o}/{r}/commits?sha={head}`, paginated,
   capped) until one is tagged — exact match → the tag, otherwise
   `"<tag>-<N>-g<short>"`. No reachable tag within the scan window → the bare
   short commit (`git describe --always`). Registries (`OpenGL-Registry`,
   `EGL-Registry`) and `google/angle` typically have no semver tags and show a
   bare commit; `Vulkan-Docs`, `Vulkan-Headers`, and `xxHash` tag and show
   `vX.Y.Z-N-gSHA`.
3. Per needed file: `GET /repos/{o}/{r}/contents/{path}?ref={commit}` → its
   **blob** SHA (and inline base64 content for files ≤1 MB). For larger files,
   follow with `GET /repos/{o}/{r}/git/blobs/{blob}`.

Everything is pinned to the commit from step 1, so even if upstream commits
mid-resolution the content we get is exactly what our metadata describes — the
metadata/data race is eliminated without the weaker "re-fetch and compare"
approach.

**Why the Contents API, not a recursive tree walk.** An earlier design used
`git/trees/{commit}?recursive=1` to resolve paths to blobs in one call. But the
recursive-tree endpoint truncates at GitHub's tree limits, and large repos
(`google/angle`, `Vulkan-Docs`) can exceed them — silently dropping the entry we
need. The Contents API resolves each file independently (one call, blob SHA plus
inline content when small) and never truncates, at the cost of one call per file
instead of one per cluster. Per-cluster sharing still applies to the HEAD/describe
resolution.

For `--lock`, the commit/describe/blob are already known, so we skip steps 1–3
and fetch each file's content directly by its pinned blob id (content-addressed,
cache-first).

### `--lock` shortcut

When generating from a `--lock` manifest, commit/describe/tree are already known.
We skip steps 1–3 and fetch each required file directly by its pinned blob id
(step 4 only), cache permitting. Cheaper and fully reproducible.

`--lock` requires that the locked blobs are obtainable. The user must therefore
either:

1. pass `--fetch` as well (blobs are fetched by id, cache-first), **or**
2. have a gloam build whose **bundled files match the locked blobs**.

gloam computes the required files for the run and compares each one's pinned
`blob` against the corresponding entry in `bundled/provenance.json`. If every
required blob matches the bundle, `--lock` works offline against bundled
content. If any differs or is absent, gloam refuses and instructs the user to
add `--fetch` — bundled content cannot satisfy that lock.

### GITHUB_TOKEN

All API calls read `$GITHUB_TOKEN` when present and fall back to unauthenticated
access otherwise.

- Unauthenticated: 60 req/hr — fine for a one-off `--fetch` thanks to per-cluster
  sharing + the cache, but not for CI.
- Authenticated: 5000 req/hr for a PAT; **1000 req/hr/repo** for the Actions
  `GITHUB_TOKEN`. Reading other public repos' APIs is allowed for any token.

In `gloam-pregen` CI, pass the automatic token — no PAT to mint or store:

```yaml
- name: Generate …
  env:
    GITHUB_TOKEN: ${{ github.token }}
  run: gloam --api … --fetch …
```

---

## Cache

Reduce API traffic and make `--fetch` pleasant offline-ish. The cache exists
**only** to serve `--fetch`; bundled-mode generation reads embedded files
directly and never touches it. Accordingly the cache implementation
(`rusqlite`/bundled SQLite) lives behind the existing `fetch` feature, so
default / `--no-default-features` builds stay lean and SQLite-free.

Location via the `directories` crate's cache dir:

- Windows: `%LOCALAPPDATA%\gloam\cache`
- Linux: `~/.cache/gloam`
- macOS: `~/Library/Caches/gloam`

### Why SQLite, not a snapshot tarball

The first design (a `.tar` snapshot of HEAD + JSON sidecars) only modeled
repeated `--fetch` against current HEAD. `--lock` breaks that assumption: it can
pin **older** commits/blobs, and using an old lock must not clobber our notion of
HEAD. We need to cache **non-HEAD** content alongside HEAD, and evict by *use*,
not by *recency of HEAD*. That calls for two independent TTL classes and
transactional eviction — a relational store fits far better than a tar we'd have
to rewrite wholesale and GC by hand.

The cache is a **single SQLite file** (still honoring the single-file
constraint), in WAL mode for safe concurrent access.

### Backend choice: `rusqlite` (bundled SQLite)

We use `rusqlite` with its `bundled` feature, behind the `fetch` feature.
Rationale:

- **No *new* toolchain cost.** The `fetch` feature already pulls `reqwest` +
  `rustls`, whose crypto provider (`ring`/`aws-lc-rs`) compiles C/assembly — so
  `--features fetch` already requires a C compiler. Bundled SQLite adds no new
  category of dependency, only ~10–30s one-time compile and ~1 MB, both
  `fetch`-only. (Default `--no-default-features` builds have neither `fetch` nor
  SQLite.)
- **Mature + inspectable + relational.** The data model is genuinely relational
  (many-to-many blob↔commit, joins, range-based eviction); SQL expresses it
  cleanly, and the file is inspectable with stock `sqlite3` / DB Browser.

Pure-Rust SQL alternatives were considered and rejected: **Turso/Limbo**
(pure-Rust, SQLite-file-compatible, but beta and churning) and **GlueSQL**
(pure-Rust but early-stage and *not* SQLite-file-compatible, losing third-party
inspectability). Their only advantage — avoiding `cc` — is already moot given
the TLS stack.

Integrity note: blobs are keyed by SHA, so any blob read from the cache is
re-hashed against its key before use; a mismatch is treated as a miss and
re-fetched. Content correctness never depends on the storage engine being
bug-free.

### Schema

```sql
-- HEAD tracking per repo; drives HEAD TTL (re-resolution), NOT eviction.
repos(repo TEXT PRIMARY KEY, branch TEXT, head_commit TEXT, head_fetched_at INTEGER)

-- Commit metadata; `describe` cached so we don't re-walk tags each time.
commits(commit_sha TEXT PRIMARY KEY, repo TEXT, describe TEXT, last_used INTEGER)

-- Many-to-many bridge: a file path at a commit resolves to a blob.
tree_entries(commit_sha TEXT, path_in_repo TEXT, blob_sha TEXT,
             PRIMARY KEY (commit_sha, path_in_repo))

-- Content, deduped by SHA. NO commit column: a blob lives in many commits.
blobs(blob_sha TEXT PRIMARY KEY, content BLOB, last_used INTEGER)
```

### Two TTL classes

- **HEAD TTL** (`repos.head_fetched_at`, default ~1 day): how long a cached HEAD
  is trusted before a no-lock `--fetch` re-resolves it. Governs *re-resolution
  only* — a stale HEAD is never evicted, just refreshed on next use.
- **Object TTL** (`commits.last_used`, `blobs.last_used`, default longer): how
  long unused commits/blobs survive before eviction. Refreshed (`last_used = now`)
  on every reuse.

### Access patterns

- **`--fetch`, no lock:** if `repos.head_fetched_at` is fresh, reuse
  `head_commit → tree_entries → blobs` with zero API calls. If stale, re-resolve
  HEAD (acquisition model); unchanged HEAD just bumps `head_fetched_at`, a moved
  HEAD inserts the new commit/tree/blobs. Old blobs stay until object-TTL
  eviction.
- **`--fetch --lock`:** never touches `repos` (HEAD). Keys off the manifest's
  pinned commit/blob SHAs: serve blobs straight from `blobs`, fetch missing ones
  by id and insert them, and opportunistically record the manifest's
  commit/`describe`/tree rows so describe stays cached. Bumps `last_used` on
  everything reused.

### Seeding from the bundle (unifies the lookup path)

In `fetch` builds, on each run gloam seeds the cache from the embedded bundle if
the content isn't already present: `INSERT OR IGNORE` the bundled blobs plus their
`commits`/`tree_entries` rows derived from `bundled/provenance.json`. This
collapses the lookup to a single path — **"in cache? → else fetch (if
`--fetch`) → else error"** — with no separate "is it bundled?" branch, because
bundled content is *in* the cache.

- Seeding writes `commits`/`tree_entries`/`blobs` but **not** `repos.head_*` —
  the bundled commit isn't necessarily HEAD, so seeding behaves like a lock, not
  a HEAD refresh.
- Seeded blobs are ordinary cache entries (evictable by object-TTL). If one ages
  out, the next run that needs it re-seeds for free from the embedded bytes —
  idempotent and self-healing. `INSERT OR IGNORE` keyed on blob SHA makes
  re-seeding a no-op when present.

**Build-configuration split:**

- **`fetch` ON:** cache exists; seed-from-bundle applies; lookups are
  cache-or-fetch as above.
- **`fetch` OFF (`--no-default-features`):** no SQLite, no cache, no network.
  Bundled files are read directly. `--lock` still works *only* if its pinned
  blobs match the bundled blobs (compare lock pins vs `bundled/provenance.json`,
  both in-binary); otherwise it errors that this build has no `--fetch`.

**Self-sufficiency invariant.** The cache rows carry everything outputs need
(repo, commit, describe, blob, path), so a run where every required file hits the
cache emits full headers and `.gloam/manifest.json` with **zero** network
access. The cache is always sufficient to regenerate all provenance metadata for
the content it holds.

### Schema versioning

The cache is pure derived data — fully rebuildable from the network and the
bundle — so there is nothing to migrate and no data to lose. We therefore do
**not** version tables or write migrations. Instead, the schema version is
stored in the file header via `PRAGMA user_version`; on startup, if it doesn't
match gloam's expected version, the database is dropped and recreated from
scratch. (Versioned table names risk orphaning old tables; migrations add
unwarranted complexity.)

### Eviction / GC

Cheap SQL, no manual walking:

```sql
DELETE FROM commits WHERE last_used + :object_ttl < :now;
DELETE FROM blobs   WHERE last_used + :object_ttl < :now;
-- then sweep orphaned tree_entries (or rely on ON DELETE CASCADE)
```

A blob is evicted purely by its own last-use, independent of any commit — correct,
since the same blob is shared across commits that didn't modify it.

### Subset projection

A cache, bundle, or `--lock` manifest may hold more than a given run needs. We
project provenance down to the files the run's outputs actually use, by the same
rules that select which copyright notices to display. One projection function
feeds the header, `--version`, and the output manifest.

---

## Embedded bundle

`bundled/` keeps its file tree and gains a checked-in **`bundled/provenance.json`**
— the canonical, human-readable, deterministic pin set — embedded via
`include_str!` and parsed at startup. (The cache is a private SQLite DB, a
separate concern; `bundled/provenance.json` is the checked-in, reviewable
artifact.) The bundle has no TTL; it is refreshed only when a maintainer
re-bundles.

`bundled/provenance.json` is the source of truth for `--version` and for bundled
(non-`--fetch`) generation. Content hashes can be recomputed from the embedded
bytes at build time to detect drift between the manifest and the files.

### `cargo xtask` bundler (replaces `scripts/fetch_bundled.sh`)

A dev-only workspace member that runs the **same acquisition path** as `--fetch`
and writes both the `bundled/` files and `bundled/provenance.json`. This removes
the shell script and guarantees bundled and fetched provenance are produced
identically. The reachability test currently in `src/fetch.rs` is reworked to
exercise the API path.

**Structural prerequisite.** To let `xtask` call gloam's acquisition code, the
repo becomes a **cargo workspace** and gloam gains a **library target**
(`src/lib.rs`) exposing the acquisition/provenance modules; `xtask` depends on
`gloam` as a lib. This does not change how the `gloam` binary builds or
publishes (`xtask` is not a dependency of `gloam`), and the lib target also makes
the acquisition + cache code directly unit-testable. Invoked via a
`.cargo/config.toml` alias: `cargo xtask bundle` (optionally `--xml` / `--hdrs`,
mirroring the old script).

### `gloam lock` — manifest-only snapshot subcommand

`gloam lock --out <file>` writes a provenance-only manifest (no loader output;
an empty `output` BOM) pinning **every supported upstream source** — the
reusable point-in-time lock referenced in the consumer guide. It shares the
loader path with `--fetch`: bundled provenance by default, or upstream HEAD with
`--fetch` (as in the pregen preflight). The name mirrors the `--lock` input
flag: `gloam lock` produces what `--lock` consumes.

Today the snapshot always covers all sources — a safe superset for any subset
`--lock`. Per-`--api` narrowing is a possible future refinement.

---

## Output scoping: per-`FeatureSet`, and the ANGLE bug

**The bug.** Today the ANGLE notice is keyed on the spec *family*
(`src/preamble.rs`: `matches!(fs.spec_name, "gl" | "egl")`) and hardcodes the
string `"(gl_angle_ext.xml, egl_angle_ext.xml)"`. So a `gl:core` loader wrongly
advertises ANGLE — and names an EGL file in a GL loader — even though no ANGLE
extension is in scope. The parser also flattens all docs into one `RawSpec`,
losing which file contributed what.

**The model.** The provenance unit is the **generated output file**, i.e. one
per `FeatureSet`. A merged `gl:core,gles2,egl` run still emits separate
`gl.{h,c}` and `egl.{h,c}`. Each `FeatureSet` carries the set of source files
that fed *it*:

```
contributing sources(FeatureSet) =
      primary spec XML for this FeatureSet
    + supplemental XMLs merged into this FeatureSet
    + required_headers emitted for this FeatureSet
```

This is coarse (file granularity, not per-symbol): a file counts if it was merged
in, even if its content was fully redundant. The header, copyright grouping, and
`output[].derived_from` all derive from this one per-`FeatureSet` set, so they
agree by construction.

**Request-aware supplemental merge.** To make `gl:core` exclude
`gl_angle_ext.xml`, the supplemental-merge decision must depend on the requested
**APIs**, not just the spec family (`gl:core` and `gles2` share `spec_name ==
"gl"`). The merge mapping:

| supplemental | merged when… |
| --- | --- |
| `glsl_exts.xml` | any GL-family API (`gl`/`gles1`/`gles2`) — it carries `supported="gl"` and `supported="gl\|gles2"` entries |
| `gl_angle_ext.xml` | a GLES API (`gles1`/`gles2`) is requested |
| `egl_angle_ext.xml` | `egl` is requested |

So `gl:core` alone merges `gl.xml` + `glsl_exts.xml` only (plus its headers); no
ANGLE. The production `gl:core,gles2,egl --merge` build merges `gl_angle_ext.xml`
into the GL loader (GLES2 in scope) and `egl_angle_ext.xml` into the EGL loader.
This is low-risk for output bytes — ANGLE's GLES-targeted extensions would not be
*selected* into a core-only loader anyway — but validate the merged production
output stays byte-identical when implementing.

This threads the requested API set into `fetch::load_spec` (today it only knows
`spec_name`).

---

## Attribution: static table × fetched facts

Keep fetched provenance purely factual (repo, commit, describe, blob). Copyright
**text** — holder, year range, license — is gloam's own static knowledge, a table
keyed by cluster. Combine the two at emit time. Years come from
`build_info::BUILD_YEAR` (deterministic per gloam release), as today.

This also closes a pre-existing gap: **xxHash currently has no attribution** even
though `xxhash.h` ships in output. Add its BSD-2-Clause notice, scoped (like every
notice) to loaders that actually emit the file.

### Deterministic ordering (must be stable)

- **Copyright notices:** gloam's MIT notice first; then one notice per distinct
  (holder, license) among contributing sources, sorted by SPDX identifier, then
  holder. (⇒ Apache-2.0 Khronos, BSD-2-Clause xxHash, BSD-3-Clause ANGLE.)
- **Sources block:** repositories sorted case-insensitively by `owner/name`
  slug; files within a repo sorted by `path_in_repo`.
- **`provenance` object:** keyed by logical file name, keys sorted ascending
  (use `IndexMap` populated in sorted order to preserve it through serde).
- **`output` array:** sorted by `path` ascending.

No `HashMap` iteration order, no timestamps, no randomness — per the project's
byte-identical-output rule.

---

## Header block assembly

Order within the comment: `Generated by … {describe}` → reproducing command line
→ `Extensions:` summary (existing logic) → copyright notices → `Generated from
the following upstream sources:` (repo-grouped, two-level) → `DO NOT EDIT`
footer. The two-level source layout (repo+describe on one line, indented
`path (blob …)` lines under it) keeps a repo-wide commit bump from churning every
file line in downstream diffs.

`DO NOT EDIT` moves to its own footer:

```
DO NOT EDIT. This file is generated by gloam and will be overwritten;
make changes to the gloam invocation or inputs, not to this file.
```

### `--version` output

`--version` reports the **embedded bundle's** provenance (from
`bundled/provenance.json`) using the same repo-grouped layout as the header
sources block — repo + `git describe` per line, indented `path (blob …)` lines
beneath — minus the C-comment asterisks. There are few enough files that listing
them all is reasonable; a compact per-repo-only mode can be added later if it
grows unwieldy. The formatter is shared with the header builder so the two never
drift.

---

## `.gloam/manifest.json`

Always written to the output root, pretty-printed (line-diffable). Three
sections — `gloam`, `provenance`, `output` — fully specified in
[manifest.md](manifest.md). Producer notes:

- `provenance` is the **pin set**, not the union of usage. On a fresh run it is
  exactly what was fetched; under `--lock` it is the input manifest's
  `provenance` carried **verbatim** (no add/remove), even if this run used a
  subset.
- `output[].derived_from` is the per-file usage (the contributing-sources set of
  that file's `FeatureSet`, or the single source file for a `verbatim` header).
- `output[].blob` and the upstream `blob` fields are git blob SHA-1s
  (`git hash-object`), reproducible and verifiable with stock git.
- `--lock` reads **only** `provenance`; `gloam` and `output` are regenerated.

---

## `gloam-pregen` workflow changes

> These are `gloam-pregen` process details, not main-repo behavior.

- Add `GITHUB_TOKEN: ${{ github.token }}` to each generation step's `env`.
- **Snapshot preflight.** Run the manifest-only subcommand once at the top of the
  job (with `--fetch`) to write a `manifest.json` at the pregen repo root pinning
  every repo at one consistent instant. Then drive all three loaders
  (`gl-egl`, `vulkan`, `vulkan-aio`) with `--lock manifest.json --fetch`. Upstream
  HEAD is resolved exactly once; the three loaders are provably generated from the
  same snapshot; subsequent blob fetches hit the cache.
  - Consequence (accepted): each loader's `.gloam/manifest.json` carries the full
    snapshot pin set **verbatim**, so e.g. `vulkan/.gloam/manifest.json` lists
    gl/egl/angle pins it never referenced. This is the intended "broad lock,
    trimmed output → unreferenced pins" behavior.
- Replace the weak commit message. Instead of a second `gloam` invocation (which
  would race the first), the commit step reads the just-written
  `.gloam/manifest.json` from each output tree and formats a human-readable
  message body from `gloam` + `provenance` (gloam version, and each upstream
  repo's describe/commit that changed). The manifest is committed alongside the
  loaders, so its diff is itself an auditable record.
- Because committed artifacts have no timestamps, the daily job only produces a
  diff (and thus a commit) when upstream content actually changed.

---

## Schema versioning

`schema_version` starts at `1`. Bump on incompatible layout changes; additive
backward-compatible fields need no bump. Consumers reject unknown majors.
