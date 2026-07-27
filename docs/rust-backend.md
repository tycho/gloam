# Rust generator backend — design notes

> **Status:** implemented (GL/GLES) and proven. The backend lives in
> `src/generator/rust/mod.rs` (`gloam … rust`); the design below is largely
> realized. It emits a self-contained `#![no_std]` crate, detects extensions
> with XXH3 (reusing the resolver's `Extension.hash`), and drives a real
> triangle on hardware — see
> [examples/rust/gl-triangle](../examples/rust/gl-triangle/). Size is at
> parity with the C loader (below). mx-global is implemented as `--mx-global`
> (free-function dispatch over a write-once `UnsafeCell` global — see the
> note below on why not `OnceLock`). Vulkan is still out of scope.

## Goal & scope

Add a **Rust generator backend** alongside the C backend, emitting a loader
that is **more idiomatic than GLAD2's Rust output** without giving up gloam's
small-code / fast-load characteristics.

- **GL / GLES first**, targeting a Rust port of the `gl-triangle` example.
- **Vulkan later** — its type system is the hard part (see
  [type translation](#type-translation-the-main-new-work)); GL is much easier.

## What to avoid (the GLAD2 reference)

A GLAD2 Rust loader (`glad --api gl:core,gles2 --merge rust --alias`) was
examined at `D:\dev\vk-api-loader-shootout\tmp` as the anti-pattern. Its
deficiencies (all of which gloam structurally avoids):

- **Emits the entire registry** (every vendor extension) → ~15k lines, ~5,400
  flat `pub const` enums. Its monolithic `load()` touches every function, so the
  whole registry is **pinned against dead-code elimination** once you load.
- **Every function is a `static mut FnPtr { ptr, is_loaded: bool }`** — 16 bytes
  each (double a raw pointer), thousands of them, as global mutable state
  (unsound under edition 2024).
- **transmute-per-call** through the global storage; unloaded slots point at a
  `not_initialized` panic stub.
- **No selection, no packed name blob, no bulk/range loading** (one
  `GetProcAddress` per function by name), **no extension hashing** (string
  compares). Release `.rlib` ≈ 7.7 MiB (an rlib overstates linked size, but the
  DCE-pinning above means the effective bloat is real).

## API shape (decisions)

### Naming — keep GL short names, own the context

- The generated type is **`Gl`** (UpperCamelCase — no lint fights); the caller's
  binding is conventionally **`gl`**; methods keep the **verbatim GL short name**.
  Call site reads `gl.DrawArrays(GL_TRIANGLES, 0, 3)` — namespace-prefix feel,
  original names, no rename transform.
- A lowercase `struct gl` was rejected: it trips `non_camel_case_types`. The
  `Gl`-type + `gl`-binding pair gives the same reading with zero lint friction.
- `.` (method on an owned value) is the default, not `::` (which implies a
  global / free function — see [dispatch modes](#dispatch-modes-mirror-c-mx--mx-global)).

### Constants & enums — newtype with free typed consts

- `#[repr(transparent)] pub struct GLenum(pub u32)` and a companion
  `GLbitfield` newtype with `BitOr`/`BitAnd` (for `GL_*_BIT` combining).
  `repr(transparent)` ⇒ ABI-identical to `u32`, zero runtime cost.
- Constants are emitted as **free consts typed as the newtype**, glob-importable:
  ```rust
  pub const GL_TRIANGLES: GLenum = GLenum(0x0004);
  // use gloam_gl::consts::*;  →  gl.DrawArrays(GL_TRIANGLES, 0, 3)  (bare name)
  ```
  This gives bare `GL_*` names (no `GLenum::` prefix) **and** call-site type
  safety (a raw integer won't coerce to `GLenum`). Associated consts
  (`GLenum::TRIANGLES`) were the alternative; identical safety, but the free-const
  form matches the "namespace prefix, not a rename" preference and the C output.
- **True per-group enums are punted indefinitely** — GL's `group=` metadata is
  too incomplete to be worth the untangling, and driver-returned values would
  risk invalid-discriminant UB.
- **Constant-typing caveat (the real cost of newtypes):** each const must be
  assigned a newtype, but some GL values are polymorphic — `GL_ZERO`, `GL_ONE`,
  `GL_NONE` are used as both enum and integer. Decision: **default those to
  `GLenum`** and revisit if real API usage forces casts. This is a coarse
  enum/bitfield/int bucketing, far smaller than full per-group typing.

### Safety — thin raw crate now, optional safe crate later

- gloam generates the **low-level crate**: raw context, PFN table, `unsafe`
  dispatch methods, loader, enums, extension detection. Mechanical from
  `FeatureSet`, in scope. Ideally `#![no_std]` (core + the loader callback).
- A **safe wrapper is a separate, hand-written, downstream crate** (the `-sys` +
  safe convention), at the user's discretion. Safe abstractions (object
  ownership, slice-vs-ptr+len, `glGetError`→`Result`, RAII) need human judgment
  that can't be reliably derived from XML. Generating mechanical safety is a
  possible *future* project, deliberately out of the initial scope.

## Context representation (perf-critical)

The rules here are what make Rust dispatch match C's `global + offset → PFN`:

1. **PFN table stored inline** as a fixed `[Pfn; K]` (`K` known at generation
   time). **Never `Box<[…]>` / `Vec<…>`.** Inline ⇒ `self.pfns[IDX]` is one load
   `[base + const_offset]`. A boxed/vec table adds a second load (pointer in the
   struct → then the PFN) — the double-indirection to avoid.
2. **`Gl` is `!Clone` and `!Copy`.** The inline table is tens of KB; deriving
   `Clone` would make `gl.clone()` a silent ~20 KB `memcpy`. Not deriving it
   forces sharing by `&Gl` / `Arc` / `static` — no accidental big copies.
3. **`Arc<Gl>` adds no dispatch load.** `Arc → &data` is constant-offset pointer
   arithmetic, not a memory load; the inline PFN is then one load. Arc's only
   cost (atomic refcount) is on clone/drop, never on the hot path. (Arc is
   likely unnecessary, but it wouldn't harm dispatch.)

Pointers are 8 bytes each (like C's `pfnArray`), not GLAD's 16-byte
`FnPtr{ptr,bool}`.

## Dispatch modes (mirror C `--mx` / `--mx-global`)

- **`--mx` (explicit context):** owned `Gl`, `gl.DrawArrays(...)` methods, share
  `&Gl`. Single indirection via the receiver register.
- **`--mx-global` (primary in gloam's philosophy), as implemented:**
  ```rust
  static GLOBAL: GlobalCell = ...;             // UnsafeCell<Gl>, statically zero-filled
  unsafe { gl::load_gl_global(loader) }?;      // or init_global(gl) with an owned Gl
  gl::DrawArrays(GL_TRIANGLES, 0, 3);          // free fn over the global — the `::` form
  ```
  Dispatch is `[fixed_base + offset] → PFN` — a single indirection from a
  link-time-fixed base, exactly like C, with **no branch and no atomic** on
  the hot path.
  - **Why not `OnceLock`** (the original sketch): the crate went `#![no_std]`
    (`OnceLock` is std-only), and its `get()` adds an is-initialized check C
    doesn't have. Instead the global is a **statically zero-filled `Gl`** in an
    `UnsafeCell`: reading flags before init is defined (everything reads
    absent), free presence queries stay *safe*, and only `init_global` is
    unsafe — its contract ("complete before, and never concurrent with, any
    other global access; publish via an ordinary happens-before edge") is the
    same discipline C's global already requires. Dispatching before init trips
    the dispatch `debug_assert` in debug builds.

## Matching C's size / load-time

Every mechanism behind gloam-C's small/fast loader has a zero-cost Rust form:

| Mechanism | Rust form |
|---|---|
| API selection (only requested APIs) | already done upstream in `resolve/` |
| 8-byte PFN slots, bulk **range loading** | inline `[Pfn; K]`, same `(start,count)` range tables |
| Packed **function-name blob** + offsets | `static FN_NAMES: &[u8]` + `[u16; K]` — same `.rodata` |
| **Extension hashing** (XXH3-64, sort, bsearch) | `static EXT_HASHES: [u64; M]` + small (vendored/no_std) xxh3 + `binary_search` — beats GLAD's string compares |
| Dispatch wrappers | `#[inline]` methods melt into call sites |
| Loader | take `&mut dyn FnMut(&CStr) -> *const c_void` (one instantiation, avoids monomorphization bloat) |
| Release footprint | same knobs: `panic="abort"`, LTO, strip |

`#[repr(C)] union` field access, if we ever want a typed named view over the
same memory, is a **pure reinterpret — no runtime type/bounds check** (the only
cost is the compile-time `unsafe`). So the C union trick is available at zero
cost, though the inline array + const-indexed `get_unchecked` methods likely
suffice.

## Architecture: where it plugs in

The resolve pipeline is backend-neutral; a Rust backend reuses all of it. See
the survey in the module map ([CONTRIBUTING.md](../CONTRIBUTING.md)).

**Reused as-is** (no changes): selection, requirements, merge batching, command
ordering optimization, PFN range tables, alias pairs, topo sort, extension
hashes, indices, **structured command params** (`Param { name, type_raw }`),
bootstrap-command policy.

**The seam** (small, additive):
- `Generator::Rust(RustArgs)` variant in `src/cli.rs` (the subcommand *is* the
  backend selector — no `--language` flag).
- `generator::rust::generate(fs, args, out, store, cmdline) -> GeneratedTree`
  mirroring `generator::c::generate`.
- A match arm in `src/main.rs` (next to the `Generator::C` arm).
- One `Generator::Rust(r) => r.alias` arm in `src/resolve/mod.rs` (the only
  existing coupling that reads the generator's alias flag).

### Type translation (the main new work)

The `FeatureSet` carries **C type text** in exactly three fields, which a Rust
backend cannot emit directly and must translate:

- `Param.type_raw` (e.g. `"const GLuint *"`)
- `Command.return_type` (e.g. `"const GLubyte *"`)
- `TypeDef.raw_c` (whole assembled C typedefs)

There is **no existing type-mapping infrastructure**. So a Rust backend needs a
**C-type → Rust-type translator**:

- **GL/GLES: small.** ~40 base types, nearly all primitive aliases —
  `GLuint→u32`, `GLint→i32`, `GLenum→GLenum` (newtype), `GLfloat→f32`,
  `GLchar→c_char`, `GLboolean→u8`, `GLsizei→i32`, `GLintptr`/`GLsizeiptr→isize`,
  pointers→`*const`/`*mut`, `GLDEBUGPROC`→fn pointer. Calling convention is
  `extern "system"` (matches `APIENTRY`). Array params (`float v[4]`) are
  smuggled inside `type_raw` with the name embedded — must be special-cased.
- **Vulkan: large.** `TypeDef.raw_c` holds full struct/union/handle/fn-pointer
  bodies; re-deriving those as Rust needs either a C-text parser or **richer IR
  exposure** (the structured struct members are flattened into `raw_c` before
  `FeatureSet`). This is the dominant reason Vulkan is deferred.

The Rust backend also re-implements the C-policy bits from
`generator/c/model.rs` (PFN-type names, parameter-string formatting) in Rust
form, and its own emit layer (templates or direct codegen).

## Results (what was proven)

The original skepticism was size/load-time parity with C. Outcome, measured on
the merged `gl:core=3.3,gles2=3.0` loader (2940 commands, 992 extensions),
compiled to an object:

| section | C `gl.o` | Rust loader | note |
|---|---|---|---|
| `.text` | 5,304 | ~850 | dispatch is `#[inline]`, so it lives at call sites (like C's macros), not in the loader object; Rust also skips the runtime Shellsort (its known table is pre-sorted) |
| `.data`/`.rodata` | 93,570 | ~92,200 | packed name blob + XXH3 table; near-identical once both carry the ext-hash table |
| `.bss` | 24,552 | 0 | owned-context mode; an `OnceLock` global would add ~24 KB, matching C |

So: **at parity, no structural bloat.** Every early regression (fat-pointer
name table; tuple-padded hash table) traced to a representation choice that was
fixed back to ≤ C. The full 2940-method surface added *nothing* to the compiled
object (inline methods materialize only when called).

Other items, all settled:

- **The C→Rust type translator holds across the entire GL/GLES surface**
  (`cargo check` clean over all 2940 signatures, incl. opaque handles, CL
  structs, all five callback typedefs).
- **Extension detection matches C** — XXH3 reused from `Extension.hash`, hashed
  at runtime with `xxhash_rust::xxh3::xxh3_64`; a mock-driver test and the live
  hardware run both confirm it.
- **Constant typing** — polymorphic `GL_ZERO`/`GL_ONE`/`GL_NONE` default to
  `GLenum`; no friction observed in the example.
- **Real rendering** — [examples/rust/gl-triangle](../examples/rust/gl-triangle/)
  draws a triangle via winit + glutin on a desktop GL 3.3 context (verified on
  an NVIDIA RTX 5090), plus a headless `--ci` pixel check.

## Deliberate omissions

- **No `--loader` (dlopen/LoadLibrary) layer.** In C that layer papers over a
  real portability gap; in Rust the established idiom is the downstream crate
  choosing [`libloading`](https://crates.io/crates/libloading) (or its
  windowing stack's `get_proc_address`, as the example does). Emitting one
  would also drag platform `dlopen` bindings into an otherwise `#![no_std]`
  crate. Intentionally omitted, not deferred.

## Open decisions

- **Emitter style: direct string emission vs templates.** The C backend
  renders minijinja templates; the Rust backend emits via `format!`/`push_str`
  with escaped literals. The string form is compiler-adjacent (no template
  runtime, rustc catches malformed interpolations) but the `\x20`-indented
  literals are harder to scan and review than `.j2` files. Fine at the
  current ~1100 lines; **revisit before Vulkan**, which would multiply the
  emit surface (structs, unions, handles, PFN typedefs).

## Deferred

- **Vulkan** — the `TypeDef.raw_c` struct/handle bodies are the hard part.
