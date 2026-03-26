# CLAUDE.md — gloam

gloam is a loader generator for Vulkan, OpenGL, OpenGL ES, EGL, GLX, and WGL.
It reads Khronos XML spec files and generates C dispatch code.

For architecture details, module map, spec gotchas, and code conventions, see
[CONTRIBUTING.md](../CONTRIBUTING.md).

## Design philosophy

**Not a GLAD drop-in.** gloam is a clean break from a GLAD fork, not an
evolution of it. GLAD compatibility is incidental, not a goal. Where GLAD
dropped the global context struct pattern (--mx-global), gloam keeps it —
`gloam_gl_context`, `gloam_vk_context`, etc. are the primary dispatch
mechanism via macro wrappers. Design for what's best, not what GLAD does.

**Performance and size matter.** The hash-based extension detection (xxHash
+ Shellsort + binary search) exists because string comparisons are a measurable
bottleneck in context initialization. Generated code size also matters. When
making tradeoffs, favor smaller generated code and faster load-time
performance.

**Deterministic, byte-identical output.** Generated loaders are checked into
downstream projects and diffs are audited after gloam changes. `IndexMap` is
used throughout for insertion-order preservation. Never introduce
non-determinism (HashMap iteration order, random seeds, etc.) and minimize
unnecessary output churn.

**C is the target.** C output is universally compatible — it works in C++
projects too. The `generator/c/` directory structure allows for future
backends, but there is no active non-C backend. Focus effort on the
C generator.

**Merged output is the primary use case.** `--merge` with GL+GLES2 is how
the loader is actually used in production (Darwinia runs on desktop GL or
GLES via ANGLE). There is significant overlap between the two APIs — aliased
extension names and many shared commands. The merged path is not a niche
feature; it's a main one.

**API breaks are okay if deliberate.** We follow semantic versioning. Breaking
changes to the generated public-facing API (function signatures, struct
layouts, macro names declared in headers) are acceptable when well-justified,
but must be a deliberate decision — not a side effect of an unrelated change.
Internal implementation details (anything not exposed in the public headers)
can change freely.

## Commit messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
specification. release-plz uses these to generate changelog entries and
determine version bumps.

```
<type>[optional scope][!]: <summary>

[optional body]
```

Common types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`.

**Breaking changes:** Use `!` before the colon (e.g. `feat!: rename context
struct`) to signal an API/ABI break in the generated output. This bumps the
minor version instead of just the patch. Do not use `!` for internal-only
changes that don't affect the public headers.

## Build & test

```sh
cargo build              # debug build
cargo build --release    # release build
cargo test               # run all tests
```

It's strongly recommended that you build with `--no-default-features` unless
you want to test/audit the `--fetch` functionality. Keeping the fetch
functionality disabled helps with reducing build time.

Requires **Rust 1.88+** (edition 2024).
