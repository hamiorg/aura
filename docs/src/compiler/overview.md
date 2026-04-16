# AURA Compiler (`compiler`)

> **The Zero-Copy Pipeline: From Human-Readable AURA to Machine-Executable Triverse Meshes.**

The compiler reads human-authored `.aura` text files and emits zero-copy `.hami`
(configuration) and `.atom` (temporal media) binary files. It is one of three Rust crates in
the Triverse system — see `engine.md` for the runtime and `history.md` for the version store.

---

## 1. System Topology: The Three-Crate Model

| Crate      | Role                                                                 |
| ---------- | -------------------------------------------------------------------- |
| `core`     | Shared data structures, ID generation, `#[repr(C)]` memory layouts   |
| `compiler` | The AURA-to-binary compiler (Lexer → Parser → Emitter)               |
| `engine`   | Execution daemon — memory-maps compiled files, runs stabbing queries |

**Crate boundary rule:** `core` is a zero-dependency library. `compiler` and
`engine` both depend on `core` but never on each other. The compiler cannot
accidentally call engine code and vice versa.

---

## 2. The Zero-Copy Lexer

The lexer scans the raw UTF-8 buffer and emits a stream of tokens. It strictly adheres to the
rule that **character escaping is prohibited**. It never allocates heap memory (`String`); it
only yields string slices (`&'a str`) tied to the source buffer's lifetime.

The lexer's hot path is a single-branch condition:

```text
if byte < 0x20 → structural token (control code)
else           → content byte (part of a key or value)
```

This enables AVX-2 SIMD vectorization: 32 bytes are evaluated per CPU clock cycle. The lexer
never allocates; it classifies and slices in-place.

**Critical invariant:** The lexer does not interpret AURA sigils (`::`, `->`, `@`, etc.). It
emits them as raw byte sequences. The parser is responsible for semantic meaning.

---

## 3. The Parser & Time Resolution

The parser consumes tokens from the lexer, tracks indentation depth for nested `::` blocks,
and builds the Abstract Syntax Tree (AST). The AST node structure mirrors the AURA document
hierarchy: namespaces contain nodes, nodes contain fields or sub-nodes.

### Time Normalization

Before emitting `.atom` files, the compiler normalizes all flexible time syntaxes into
explicit float triples:

| Source syntax       | Parsed as          | Will emit          |
| ------------------- | ------------------ | ------------------ |
| `22s~1m10s`         | start=22, end=70   | [22.0, 70.0, 48.0] |
| `22s+48s`           | start=22, dur=48   | [22.0, 70.0, 48.0] |
| `[22s, 1m10s, 48s]` | explicit triple    | [22.0, 70.0, 48.0] |
| `@time/1m32s`       | point anchor       | [92.0, 92.0, 0.0]  |

The compiler enforces the mathematical invariant: `start + duration = end`. If all three are
provided and violate the invariant, the compiler raises a hard error. If only two are provided,
the third is derived.

### Reference Resolution

The parser collects all `@domain/id` references and resolves them in a two-phase pass:

1. **First pass:** Build the local symbol table — all defined nodes and IDs in this file and
   its `info/` and `meta/` dependencies.
2. **Second pass:** Resolve all `@` references against the local table, then the catalog
   table, then the global cloud registry. Unresolved references are stored as forward arcs and
   flagged as warnings unless the `strict` directive is set, in which case they are compile
   errors.

---

## 4. The Emitter

The emitter transforms the resolved AST into binary output.

### `.hami` Emitter

1. Replaces AURA sigils (`::`, `->`, `@`, `|`) with ASCII control codes
   (`US 0x1F`, `RS 0x1E`, `GS 0x1D`, `FS 0x1C`).
2. Writes all key-value pairs as the contiguous Lexical Data Region.
3. Calculates byte offsets for every namespace and field.
4. Appends the fixed-width B-Tree Positional Index to the end of the file.

The B-Tree index is written last so the emitter can calculate all offsets in a single forward
pass over the Lexical Data Region without backpatching.

### `.atom` Emitter

1. Flattens the hierarchical AST into a contiguous array of interval structs.
2. Each struct has six 32-bit floats: `[low, high, duration, max, data_ptr, node_class]`.
3. Structs are ordered by interval `low` value for BST layout.
4. The `max` field is calculated in a second pass over the array (augmented interval tree
   property).

The flat-array layout is optimized for AVX-2 SIMD: one 256-bit register loads 8 × 32-bit
floats, covering the `low`, `high`, and `duration` fields of two adjacent nodes in a single
cycle.

---

## 5. Compilation Workflows

### A. Default (Working Draft)

```sh
aura compile
```

Ignores `.history/` entirely. Parses only the current working `.aura` files and emits
lightweight `.atom` / `.hami` artifacts. No historical overhead.

### B. On-Demand Historical Target

```sh
aura compile --take tx3ab7k
```

Reconstructs the virtual source for take `tx3ab7k` **in memory** by replaying the delta chain
from origin. Parses the reconstruction and emits compiled binaries — without writing a new
`.aura` file to disk. The working draft is untouched.

### C. Embedded History (For Runtime Querying)

```sh
aura compile --embed-history
```

Translates `SourceDelta` objects from `.history/` into `HistoryNode` objects (class `0x14`)
and embeds them into the `.atom` interval tree. The engine can then resolve `@history/v1.0`
queries at runtime. This mode produces larger `.atom` files and is intended for archival tools,
not the lightweight client SDK.

---

## 6. Compilation Exclusions

The compiler respects the exclusion list in `configs/ignore.aura`. Files and folders listed
there are skipped entirely. The `.history/` tracker also observes this list.

The `configs/` folder itself is **never compiled**. Its files (`llm.aura`, `stores.aura`,
`account.aura`, `ignore.aura`) are toolchain configuration — read only by the CLI, never
passed to the AURA-to-ATOM compiler pipeline.

---

*AURA Compiler (`compiler`) — v0.1*
*Zero-copy lexer → parser → emitter pipeline*
*Outputs: `.atom` interval trees and `.hami` B-Tree indices*
