//! Spec-agnostic emission helpers shared by the per-spec loader modules: the
//! packed command-name/offset tables, PFN range tables, the sorted XXH3
//! extension-hash tables, the `no-error` `__missing` pair, per-command
//! dispatch-method emission, and the presence-query helpers.

use std::fmt::Write as _;

use anyhow::{Result, bail};

use crate::parse::ctype::TypeRef;
use crate::resolve::{Command, FeatureSet};

use super::abi::sanitize_ident;

/// Emit the two `__missing` variants behind the `no-error` cargo feature: the
/// default panics by name (looked up in the existing name blob — no new data),
/// the `no-error` build collapses the dispatch match back to an unchecked call.
pub(super) fn emit_missing_helpers(s: &mut String, api_label: &str) {
    s.push_str(
        "// ── Unloaded-call handling ──────────────────────────────────\n\
         /// Reached when a dispatch wrapper finds a null PFN: the function is not\n\
         /// loaded (feature/extension absent, or the context was never loaded).\n\
         /// Panics with the function's name from the name blob.\n\
         #[cfg(not(feature = \"no-error\"))]\n\
         #[cold]\n\
         #[inline(never)]\n\
         unsafe fn __missing(idx: usize) -> ! {\n\
         \x20   let off = FN_NAME_OFFSETS[idx] as usize;\n\
         \x20   let name = CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_or(c\"?\");\n\
         \x20   panic!(\n\
         \x20       \"{} is not loaded (unsupported by this context, or called before load)\",\n\
         \x20       name.to_str().unwrap_or(\"?\")\n\
         \x20   )\n\
         }\n\n\
         /// `no-error` build (the KHR_no_error analogue): promise the compiler the\n\
         /// null case is impossible, so the dispatch match compiles to an\n\
         /// unchecked call.  Calling an unloaded function is undefined behavior,\n\
         /// exactly as in C.\n\
         #[cfg(feature = \"no-error\")]\n\
         #[inline(always)]\n\
         unsafe fn __missing(_idx: usize) -> ! {\n",
    );
    let _ = write!(
        s,
        "    debug_assert!(false, \"unloaded {api_label} function called in a no-error build\");\n\
         \x20   unsafe {{ core::hint::unreachable_unchecked() }}\n\
         }}\n\n"
    );
}

/// Parse a pre-baked hash literal (`"0x…"`, as the resolver stores in
/// `Extension.hash`) into a `u64`.
fn parse_hash(literal: &str) -> u64 {
    let t = literal.trim();
    let hex = t
        .strip_prefix("0x")
        .or_else(|| t.strip_prefix("0X"))
        .unwrap_or(t);
    let hex: String = hex.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    u64::from_str_radix(&hex, 16).unwrap_or(0)
}

/// Emit the extension count, the pre-baked `(hash, extArray-index)` table sorted
/// by hash for binary search, and the runtime `__ext_hash`.  Reuses the XXH3-64
/// hashes the resolver already computed (`Extension.hash`), so the Rust and C
/// loaders detect identically.  Errors on a collision (a correctness guard).
pub(super) fn emit_extension_tables(fs: &FeatureSet, s: &mut String) -> Result<()> {
    let m = fs.extensions.len();
    let _ = writeln!(
        s,
        "// ── Extensions ──────────────────────────────────────────────\n\
         pub const EXT_COUNT: usize = {m};\n"
    );

    let mut table: Vec<(u64, u16)> = fs
        .extensions
        .iter()
        .map(|e| (parse_hash(&e.hash), e.index))
        .collect();
    table.sort_by_key(|&(h, _)| h);
    for w in table.windows(2) {
        if w[0].0 == w[1].0 {
            bail!(
                "XXH3-64 collision between extensions (extArray indices {} and {})",
                w[0].1,
                w[1].1
            );
        }
    }

    // Parallel arrays rather than `[(u64, u16); N]`: a tuple would pad each
    // entry to 16 bytes (6 wasted), which for ~1000 extensions is several KiB.
    // Parallel arrays (not `[(u64, u16); N]`, which would pad each entry to 16
    // bytes): the sorted hashes, and the extArray index each hash belongs to.
    s.push_str("// XXH3-64 of each extension name, sorted for binary search.\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static EXT_HASH_KEYS: [u64; EXT_COUNT] = [");
    for (h, idx) in &table {
        let name = fs
            .extensions
            .get(*idx as usize)
            .map_or("", |e| e.name.as_str());
        let _ = writeln!(s, "    0x{h:016x}, // {name}");
    }
    s.push_str("];\n");
    s.push_str("// extArray index for the correspondingly-ranked EXT_HASH_KEYS entry.\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static EXT_HASH_IDX: [u16; EXT_COUNT] = [");
    for chunk in table.chunks(20) {
        let line: Vec<String> = chunk.iter().map(|(_, idx)| idx.to_string()).collect();
        let _ = writeln!(s, "    {},", line.join(", "));
    }
    s.push_str("];\n\n");

    Ok(())
}

/// Emit the packed function-name blob (one NUL-separated `&[u8]` in `.rodata`,
/// like the C backend's `kFnNameData`), a parallel offset table, and the PFN
/// range tables that `load()` walks.  Packing avoids a per-name pointer +
/// relocation (a `[&CStr; K]` table costs a 16-byte fat pointer and a
/// load-time fixup per command).
pub(super) fn emit_command_tables(fs: &FeatureSet, s: &mut String) {
    let k = fs.commands.len();
    let f = fs.features.len();
    let _ = writeln!(
        s,
        "// ── Command table ───────────────────────────────────────────\n\
         pub const COMMAND_COUNT: usize = {k};\n\
         pub const FEATURE_COUNT: usize = {f};\n"
    );

    // Record each command name's start offset in the blob.  The offset type is
    // the narrowest that indexes it.
    let mut offsets: Vec<usize> = Vec::with_capacity(k);
    let mut total = 0usize;
    for c in &fs.commands {
        offsets.push(total);
        total += c.name.len() + 1;
    }
    let off_ty = if offsets.last().copied().unwrap_or(0) <= u16::MAX as usize {
        "u16"
    } else {
        "u32"
    };

    // One NUL-terminated name per source line, joined at compile time by the
    // `\`-continuations (each strips the newline + indent of the next line).
    // Rust can't interleave comments inside a literal like C's blob, so the
    // name↔index↔offset annotations live on FN_NAME_OFFSETS below.
    s.push_str("#[rustfmt::skip]\n");
    s.push_str("static FN_NAME_DATA: &[u8] = b\"\\\n");
    for c in &fs.commands {
        let _ = writeln!(s, "    {}\\0\\", c.name);
    }
    s.push_str("\";\n\n");

    s.push_str("// Byte offset of each command name in FN_NAME_DATA, indexed in\n");
    s.push_str("// lockstep with the pfn table (slot [i] == command i).\n");
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static FN_NAME_OFFSETS: [{off_ty}; COMMAND_COUNT] = [");
    for (i, c) in fs.commands.iter().enumerate() {
        let _ = writeln!(s, "    {:>7}, // [{i}] {}", offsets[i], c.name);
    }
    s.push_str("];\n\n");

    // (feature/extension index, first pfn slot, count) triples.  Extension
    // ranges are per-API so each `load_<api>` loads only its own.
    emit_range_table(s, "FEATURE_RANGES", &fs.feature_pfn_ranges, |i| {
        fs.features
            .get(i as usize)
            .map_or_else(String::new, |f| f.full_name.clone())
    });
    for (api, ranges) in &fs.ext_pfn_ranges {
        emit_range_table(s, &format!("EXT_RANGES_{api}"), ranges, |i| {
            fs.extensions
                .get(i as usize)
                .map_or_else(String::new, |e| e.name.clone())
        });
    }
}

/// Emit a `(feature/ext index, first pfn slot, count)` range table, one entry
/// per line with a `// <name>` comment (`label` maps the index to its
/// feature/extension name), under `#[rustfmt::skip]` to keep the alignment.
fn emit_range_table(
    s: &mut String,
    name: &str,
    ranges: &[crate::resolve::PfnRange],
    label: impl Fn(u16) -> String,
) {
    s.push_str("#[rustfmt::skip]\n");
    let _ = writeln!(s, "static {name}: [(u16, u16, u16); {}] = [", ranges.len());
    for r in ranges {
        let _ = writeln!(
            s,
            "    ({:>4}, {:>4}, {:>4}), // {}",
            r.extension,
            r.start,
            r.count,
            label(r.extension)
        );
    }
    s.push_str("];\n\n");
}

/// Emit an `#[inline] pub fn <name>(&self) -> bool` presence query reading
/// `self.<field>[idx]`.  Plain indexing: the index is a literal below the
/// array length, so the bounds check is statically elided — no `unsafe`.
pub(super) fn emit_flag_query(
    s: &mut String,
    name: &str,
    full: &str,
    field: &str,
    idx: u16,
    verb: &str,
) {
    let _ = write!(
        s,
        "\n    /// Whether the driver {verb} `{full}`.\n\
         \x20   #[inline]\n\
         \x20   pub fn {name}(&self) -> bool {{\n\
         \x20       self.{field}[{idx}]\n\
         \x20   }}\n"
    );
}

/// Index of a command by full name (`glGetString`, ...), if present.
pub(super) fn cmd_index(fs: &FeatureSet, name: &str) -> Option<u16> {
    fs.commands.iter().find(|c| c.name == name).map(|c| c.index)
}

/// Method name for an extension presence query: the short name, `_`-prefixed if
/// it would otherwise start with a digit (e.g. `3DFX_multisample`).
pub(super) fn method_name(short: &str) -> String {
    if short.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{short}")
    } else {
        short.to_string()
    }
}

/// Emit one `#[inline] unsafe fn` dispatch wrapper for `cmd`.  The PFN slot is
/// transmuted to `Option<fn>` (guaranteed null-pointer-optimized), never to a
/// bare `fn`: a bare fn pointer has a non-null validity invariant, so the
/// transmute itself would be UB for an unloaded slot, even if never called.
/// `safety_doc` is the `# Safety` line; `map` prints each `TypeRef` in the
/// backend's spec-appropriate spelling.
pub(super) fn emit_method(
    cmd: &Command,
    safety_doc: &str,
    map: &dyn Fn(&TypeRef) -> String,
) -> String {
    let params: Vec<(String, String)> = cmd
        .params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let name = if p.name.is_empty() {
                format!("arg{i}")
            } else {
                sanitize_ident(&p.name)
            };
            (name, map(&p.ty))
        })
        .collect();

    let ret = if cmd.return_ty.is_void() {
        None
    } else {
        Some(map(&cmd.return_ty))
    };
    let ret_sig = ret.as_ref().map(|r| format!(" -> {r}")).unwrap_or_default();

    let recv_and_args = if params.is_empty() {
        "&self".to_string()
    } else {
        let list = params
            .iter()
            .map(|(n, t)| format!("{n}: {t}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("&self, {list}")
    };
    let fn_param_types = params
        .iter()
        .map(|(_, t)| t.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let call_args = params
        .iter()
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "    /// # Safety\n\
         \x20   /// {safety_doc}\n\
         \x20   #[inline]\n\
         \x20   pub unsafe fn {short}({recv_and_args}){ret_sig} {{\n\
         \x20       let __pfn: Option<unsafe extern \"system\" fn({fn_param_types}){ret_sig}> =\n\
         \x20           unsafe {{ core::mem::transmute(*self.pfns.get_unchecked({idx})) }};\n\
         \x20       let __pfn = match __pfn {{\n\
         \x20           Some(__f) => __f,\n\
         \x20           None => unsafe {{ __missing({idx}) }},\n\
         \x20       }};\n\
         \x20       unsafe {{ __pfn({call_args}) }}\n\
         \x20   }}",
        short = cmd.short_name,
        idx = cmd.index,
    )
}

/// Emit the `--alias` pair table: `(canonical, secondary)` command indices,
/// one entry per line with the command names as a comment.  No-op when the
/// build has no alias pairs.
pub(super) fn emit_alias_pairs_table(fs: &FeatureSet, s: &mut String) {
    if fs.alias_pairs.is_empty() {
        return;
    }
    let _ = writeln!(
        s,
        "// (canonical, secondary) command indices propagated by --alias.\n\
         #[rustfmt::skip]\n\
         static ALIAS_PAIRS: [(u16, u16); {}] = [",
        fs.alias_pairs.len()
    );
    for p in &fs.alias_pairs {
        let cn = fs
            .commands
            .get(p.canonical as usize)
            .map_or("", |c| c.name.as_str());
        let sn = fs
            .commands
            .get(p.secondary as usize)
            .map_or("", |c| c.name.as_str());
        let _ = writeln!(
            s,
            "    ({:>4}, {:>4}), // {cn} <-> {sn}",
            p.canonical, p.secondary
        );
    }
    s.push_str("];\n\n");
}

/// Emit the `resolve_aliases` method: propagate each loaded pointer to its
/// unloaded alias slot (an empty body when the build has no alias pairs, so
/// the load flow can call it unconditionally).
pub(super) fn emit_resolve_aliases_method(s: &mut String, has_alias: bool) {
    if has_alias {
        s.push_str(
            "\n    /// Propagate each loaded pointer to its unloaded alias slot.\n\
             \x20   fn resolve_aliases(&mut self) {\n\
             \x20       for &(ci, si) in ALIAS_PAIRS.iter() {\n\
             \x20           let (c, d) = (self.pfns[ci as usize], self.pfns[si as usize]);\n\
             \x20           if c.is_null() && !d.is_null() {\n\
             \x20               self.pfns[ci as usize] = d;\n\
             \x20           } else if !c.is_null() && d.is_null() {\n\
             \x20               self.pfns[si as usize] = c;\n\
             \x20           }\n\
             \x20       }\n\
             \x20   }\n",
        );
    } else {
        s.push_str("\n    fn resolve_aliases(&mut self) {}\n");
    }
}

/// Emit the `load_range` method: resolve one contiguous run of PFN slots by
/// name through the caller's loader.
pub(super) fn emit_load_range_method(s: &mut String) {
    s.push_str(
        "\n    #[inline]\n\
         \x20   unsafe fn load_range(\n\
         \x20       &mut self,\n\
         \x20       loader: &mut dyn FnMut(&CStr) -> *const c_void,\n\
         \x20       start: u16,\n\
         \x20       count: u16,\n\
         \x20   ) {\n\
         \x20       for i in start..start + count {\n\
         \x20           let idx = i as usize;\n\
         \x20           let off = FN_NAME_OFFSETS[idx] as usize;\n\
         \x20           let name =\n\
         \x20               unsafe { CStr::from_bytes_until_nul(&FN_NAME_DATA[off..]).unwrap_unchecked() };\n\
         \x20           self.pfns[idx] = loader(name);\n\
         \x20       }\n\
         \x20   }\n",
    );
}

/// Emit the `hash_ext_words` method: tokenize a NUL-terminated,
/// space-separated extension list, hash each word, and flag matches against
/// the pre-baked table.  Shared by every spec whose extension report is a
/// string (EGL's eglQueryString, WGL's wglGetExtensionsString*, GLX's
/// glXQueryExtensionsString).
pub(super) fn emit_hash_ext_words_method(s: &mut String) {
    s.push_str(
        "\n    /// Tokenize a NUL-terminated, space-separated extension list and\n\
         \x20   /// set the flag for every known name (XXH3 + binary search — the\n\
         \x20   /// same pre-baked hashes the C loader uses).\n\
         \x20   unsafe fn hash_ext_words(&mut self, p: *const c_char) {\n\
         \x20       let bytes = unsafe { CStr::from_ptr(p) }.to_bytes();\n\
         \x20       for word in bytes.split(|&b| b == b' ') {\n\
         \x20           if word.is_empty() {\n\
         \x20               continue;\n\
         \x20           }\n\
         \x20           let h = xxhash_rust::xxh3::xxh3_64(word);\n\
         \x20           if let Ok(pos) = EXT_HASH_KEYS.binary_search(&h) {\n\
         \x20               self.ext[EXT_HASH_IDX[pos] as usize] = true;\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

/// Emit one free-function dispatch wrapper delegating to the method of the
/// same name on the global context.  `ctx_ty` names the context type for the
/// safety cross-reference; `map` prints each `TypeRef` in the backend's
/// spec-appropriate spelling (as in [`emit_method`]).
pub(super) fn emit_global_fn(
    cmd: &Command,
    ctx_ty: &str,
    map: &dyn Fn(&TypeRef) -> String,
) -> String {
    let params: Vec<(String, String)> = cmd
        .params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let name = if p.name.is_empty() {
                format!("arg{i}")
            } else {
                sanitize_ident(&p.name)
            };
            (name, map(&p.ty))
        })
        .collect();

    let ret = if cmd.return_ty.is_void() {
        None
    } else {
        Some(map(&cmd.return_ty))
    };
    let ret_sig = ret.as_ref().map(|r| format!(" -> {r}")).unwrap_or_default();
    let sig_params = params
        .iter()
        .map(|(n, t)| format!("{n}: {t}"))
        .collect::<Vec<_>>()
        .join(", ");
    let call_args = params
        .iter()
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "/// # Safety\n\
         /// See [`{ctx_ty}::{short}`]; the global context must be initialized.\n\
         #[inline]\n\
         pub unsafe fn {short}({sig_params}){ret_sig} {{\n\
         \x20   unsafe {{ global().{short}({call_args}) }}\n\
         }}",
        short = cmd.short_name,
    )
}

/// Emit a free presence query over the global context.  Safe: the zeroed
/// pre-init global reads as "nothing present".
pub(super) fn emit_global_flag_query(s: &mut String, name: &str, full: &str, verb: &str) {
    let _ = write!(
        s,
        "\n/// Whether the global context's driver {verb} `{full}`.\n\
         #[inline]\n\
         pub fn {name}() -> bool {{\n\
         \x20   global().{name}()\n\
         }}\n"
    );
}
