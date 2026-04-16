# Compiler Structure Reference

> **Crate layout, module tree, shared data types, pipeline stages, and output conventions.**

This document covers the physical structure of the AURA compiler system — how the three
crates are laid out on disk, what each module owns, how data flows between pipeline stages,
and what the emitted binary files look like internally.

---

## Part I — Workspace Layout

The AURA toolchain is a Cargo workspace. All three crates live under a single workspace root.

```text
aura/                               <- workspace root
  Cargo.toml                        <- workspace manifest (members = [core, compiler, engine])
  Cargo.lock

  core/                             <- shared data types and ID generator
    Cargo.toml
    src/
      lib.rs
      id.rs                         <- ID generation and prefix registry
      node.rs                       <- AtomNode, HamiNode, AtlasNode structs (#[repr(C)])
      interval.rs                   <- Allen interval triple: [low, high, duration]
      delta.rs                      <- SourceDelta, TakeObject, MarkEntry, StreamPointer
      vocab.rs                      <- VocabNode (genre, role, mood slugs)
      person.rs                     <- PersonNode, AnnotatorNode structs
      asset.rs                      <- ArtNode, MotionNode, TrailerNode structs
      entity.rs                     <- StudioNode, LabelNode structs
      availability.rs               <- WatchNode, BuyNode, RentNode, DownloadNode structs
      access.rs                     <- AccessLevel enum (open, locked, gated, embargoed, etc.)
      history.rs                    <- HistoryNode, delta chain types

  compiler/                         <- AURA source → binary emitter (aura compile)
    Cargo.toml
    src/
      main.rs                       <- CLI entry point
      lib.rs
      lexer/
        mod.rs
        token.rs                    <- Token enum: Sigil, Key, Value, Indent, Newline
        scanner.rs                  <- zero-copy byte scanner; yields &'a str slices
      parser/
        mod.rs
        ast.rs                      <- ASTNode tree: Namespace, Field, Reference, Literal
        resolver.rs                 <- two-phase @domain/id reference resolution
        time.rs                     <- time expression normalizer → [low, high, duration]
        inherit.rs                  <- >> (inherits) arc expander
      emitter/
        mod.rs
        hami.rs                     <- HAMI B-Tree emitter (manifests, people, vocab)
        atom.rs                     <- ATOM flat-array interval tree emitter
        atlas.rs                    <- ATLAS DTW alignment file emitter
      namespace/
        mod.rs
        loader.rs                   <- namespace.aura reader; builds project symbol table
        export.rs                   <- exports:: block resolver
      history/
        mod.rs
        store.rs                    <- .history/ object store reader/writer
        delta.rs                    <- SourceDelta diff engine (AST node-level diffs)
        replay.rs                   <- delta chain replayer → virtual source reconstruction
      config/
        mod.rs
        loader.rs                   <- configs/ folder reader (never compiled)
        ignore.rs                   <- ignore.aura exclusion list
      error.rs                      <- CompileError, DiagnosticLevel, Span
      directives.rs                 <- schema:: and directives:: block processor

  engine/                           <- execution daemon (aura serve / aura query)
    Cargo.toml
    src/
      main.rs
      lib.rs
      mount.rs                      <- mmap mount for .atom and .hami files
      query.rs                      <- stabbing query: SIMD interval tree traversal
      ocpn.rs                       <- OCPN marking vector M and support sub-vector S
      filter.rs                     <- node_class bitmask filter applied during SIMD loop
      arc.rs                        <- :: relational arc resolution
      verb.rs                       <- DML verbs: Fetch, Spawn, Purge, Mutate, Link, etc.
      history.rs                    <- in-engine @history/take-id resolution (read-only)
      cache.rs                      <- L1 in-process mmap cache management
      eventbus.rs                   <- support signal dispatcher (mood, rights, ad cue)
```

---

## Part II — Core Crate Data Types

The `core` crate defines every shared struct with `#[repr(C)]` so the compiler and engine
see identical memory layouts. No business logic lives here — only data.

### AtomNode

The fundamental unit of the `.atom` flat array. Six contiguous 32-bit fields.

```rust
#[repr(C)]
pub struct AtomNode {
    pub low:        f32,   // interval start (seconds)
    pub high:       f32,   // interval end (seconds)
    pub duration:   f32,   // high - low (pre-computed for SIMD)
    pub max:        f32,   // max high in subtree (augmented interval tree property)
    pub data_ptr:   u32,   // byte offset into the .hami companion file
    pub node_class: u32,   // class byte: 0x01 content, 0x02 segment, ... 0x1D download
}
```

Size: 24 bytes. One AVX-2 register (256-bit) holds 10.67 AtomNodes — in practice the SIMD
loop processes 8-node blocks, covering `low`, `high`, and `duration` of two nodes per cycle.

### Interval Triple

The canonical time representation everywhere in the system.

```rust
#[repr(C)]
pub struct Interval {
    pub low:      f32,   // start offset in seconds
    pub high:     f32,   // end offset in seconds
    pub duration: f32,   // high - low  (invariant: low + duration == high)
}
```

All time expressions in AURA source normalize to this triple before emission:

| AURA source         | low  | high | duration |
| ------------------- | ---- | ---- | -------- |
| `22s~1m10s`         | 22.0 | 70.0 | 48.0     |
| `22s+48s`           | 22.0 | 70.0 | 48.0     |
| `[22s, 1m10s, 48s]` | 22.0 | 70.0 | 48.0     |
| `@time/1m32s`       | 92.0 | 92.0 | 0.0      |

### PersonNode

```rust
#[repr(C)]
pub struct PersonNode {
    pub id:      [u8; 7],          // e.g., "p4xt9k2"
    pub first:   StringRef,        // given name → .hami string pool
    pub middle:  Option<StringRef>,
    pub last:    Option<StringRef>,
    pub screen:  Option<StringRef>, // short on-screen label
    pub legal:   Option<StringRef>,
    pub kind:    PersonKind,       // artist, actor, director, host, etc.
}

pub enum PersonKind {
    Artist,
    Actor,
    Director,
    Host,
    Narrator,
    Composer,
    Producer,
    Other,
}
```

### AnnotatorNode

Annotators are the humans who write and maintain AURA files. They share the
`p` prefix with person IDs but are stored in a separate index.

```rust
#[repr(C)]
pub struct AnnotatorNode {
    pub id:      [u8; 7],          // e.g., "p9xb3mn" (same p prefix as person)
    pub name:    StringRef,        // display name → .hami string pool
    pub roles:   AnnotatorRoles,   // bitfield: transcriber | editor | translator
    pub country: [u8; 2],          // ISO 3166-1 alpha-2
    pub contact: Option<StringRef>,// email or contact URI
}

pub struct AnnotatorRoles(u8);     // bitfield flags
impl AnnotatorRoles {
    pub const TRANSCRIBER: u8 = 0x01;
    pub const EDITOR:      u8 = 0x02;
    pub const TRANSLATOR:  u8 = 0x04;
    pub const ANNOTATOR:   u8 = 0x08;
}
```

### ArtNode / MotionNode / TrailerNode

```rust
#[repr(C)]
pub struct ArtNode {
    pub id:    [u8; 8],    // e.g., "ar4xab3c"
    pub kind:  ArtKind,    // square, landscape, 16:9, 4:3, 9:16, 2:3, custom, etc.
    pub url:   StringRef,  // cloud URL — no local file path
    pub note:  Option<StringRef>,
}

#[repr(C)]
pub struct MotionNode {
    pub id:       [u8; 8],
    pub kind:     MotionKind,  // album-motion, episode-motion, movie-motion, etc.
    pub url:      StringRef,   // cloud URL
    pub duration: f32,         // seconds
    pub loop_:    bool,        // live = loops, dark = plays once
    pub ratio:    ArtKind,     // reuses aspect ratio enum
}

#[repr(C)]
pub struct TrailerNode {
    // Inherits all MotionNode fields; kind uses TrailerKind enum
    pub id:       [u8; 8],
    pub kind:     TrailerKind, // movie-trailer, episode-trailer, podcast-trailer, etc.
    pub url:      StringRef,
    pub duration: f32,
    pub loop_:    bool,
    pub ratio:    ArtKind,
    pub released: Option<u32>, // Unix date of release
}
```

### StudioNode / LabelNode

```rust
#[repr(C)]
pub struct StudioNode {
    pub id:      [u8; 8],          // "st" prefix
    pub name:    StringRef,
    pub kind:    StudioKind,       // film, television, animation, music, etc.
    pub country: [u8; 2],          // ISO 3166-1 alpha-2
    pub parent:  Option<[u8; 8]>,  // parent studio ID (ownership hierarchy arc)
    pub logo:    Option<[u8; 8]>,  // @art/id reference
}

#[repr(C)]
pub struct LabelNode {
    pub id:      [u8; 8],          // "lb" prefix
    pub name:    StringRef,
    pub kind:    LabelKind,        // major, independent, imprint, publisher, distributor
    pub country: [u8; 2],
    pub parent:  Option<[u8; 8]>,  // parent label ID (ownership hierarchy arc)
}
```

### Availability Nodes

```rust
#[repr(C)]
pub struct WatchNode {
    pub id:        [u8; 8],
    pub platform:  StringRef,
    pub url:       StringRef,
    pub territory: StringRef,
    pub quality:   QualityFlags,  // bitfield: 4k | hd | sd
    pub access:    AccessLevel,
}

// BuyNode and RentNode share WatchNode fields plus:
pub struct BuyNode {
    // ... WatchNode fields ...
    pub price:    StringRef,  // "14.99 USD"
    pub currency: [u8; 3],   // ISO 4217
}

pub struct RentNode {
    // ... BuyNode fields ...
    pub window: StringRef,   // "30d", "48h"
}

pub struct DownloadNode {
    // ... WatchNode fields plus:
    pub quality: StringRef,   // lossless, hd, sd
    pub format:  StringRef,   // flac | mp3 | aac
    pub drm:     bool,        // live = DRM, dark = DRM-free
}
```

---

## Part III — Compiler Pipeline Stages

```text
.aura source file
        │
        ▼
  ┌─────────────┐
  │   Lexer     │  Scans raw UTF-8 bytes.
  │  (scanner)  │  Emits zero-copy &str token stream.
  └─────────────┘  No heap allocation. No string escaping.
        │
        ▼
  ┌─────────────┐
  │   Parser    │  Consumes token stream.
  │   (ast.rs)  │  Tracks indentation depth for :: blocks.
  └─────────────┘  Builds typed AST (Namespace → Field → Value).
        │
        ▼
  ┌─────────────┐
  │  Namespace  │  Reads namespace.aura at project root.
  │   Loader    │  Builds project symbol table for reference resolution.
  └─────────────┘
        │
        ▼
  ┌─────────────┐
  │  Resolver   │  Two-phase @domain/id reference pass.
  │ (resolver)  │  Local → catalog → global cloud. Forward arc warnings.
  └─────────────┘
        │
        ▼
  ┌─────────────┐
  │   Time      │  Normalizes all AURA time syntax to [low, high, duration].
  │ Normalizer  │  Enforces low + duration == high invariant.
  └─────────────┘
        │
        ▼
  ┌─────────────┐
  │  >> Expander│  Resolves inheritance arcs.
  │ (inherit)   │  Merges parent node fields into child AST nodes.
  └─────────────┘
        │
        ▼
  ┌─────────────────────────────────┐
  │            Emitter              │
  │                                 │
  │  hami.rs  → .hami (manifest)    │  HAMI: B-Tree positional index over key-value regions
  │  atom.rs  → .atom (sync mesh)   │  ATOM: augmented interval tree flat-array
  │  atlas.rs → .atlas (alignment)  │  ATLAS: DTW warp path for variant alignment
  └─────────────────────────────────┘
```

---

## Part IV — Output File Formats

### `.hami` — HAMI Manifest

HAMI replaces human-readable AURA sigils with ASCII control codes:

| AURA sigil | ASCII control code | Hex    | Name                    |
| ---------- | ------------------ | ------ | ----------------------- |
| `::`       | `US`               | `0x1F` | Unit Separator          |
| `->`       | `RS`               | `0x1E` | Record Separator        |
| `\|`       | `GS`               | `0x1D` | Group Separator (union) |
| `@`        | `FS`               | `0x1C` | File Separator          |

File layout:

```text
┌─────────────────────────────────────────────────────┐
│  HAMI Magic: "HAMI" (4 bytes)                       │
│  Version: u16                                       │
│  Root namespace offset: u32                         │
├─────────────────────────────────────────────────────┤
│  Lexical Data Region                                │
│  (contiguous key RS value US key RS value US ...)   │
├─────────────────────────────────────────────────────┤
│  B-Tree Positional Index                            │
│  (key → byte offset pairs, sorted, fixed-width)     │
└─────────────────────────────────────────────────────┘
```

The B-Tree index is appended last so the emitter calculates all offsets in a single
forward pass without backpatching.

### `.atom` — ATOM Interval Tree

ATOM is a contiguous flat array of `AtomNode` structs ordered by `low`:

```text
┌────────────────────────────────────────────────────────┐
│  ATOM Magic: "ATOM" (4 bytes)                          │
│  Version: u16                                          │
│  Node count: u32                                       │
├────────────────────────────────────────────────────────┤
│  AtomNode[0]  { low, high, duration, max, ptr, class } │
│  AtomNode[1]  { ... }                                  │
│  AtomNode[N]  { ... }                                  │
├────────────────────────────────────────────────────────┤
│  String Pool                                           │
│  (null-terminated UTF-8 strings; data_ptr indexes here)│
└────────────────────────────────────────────────────────┘
```

`max` values are filled by a second pass after initial flat-array construction:

```text
for i = N-1 downto 0:
    nodes[i].max = max(nodes[i].high, nodes[left(i)].max, nodes[right(i)].max)
```

### `.atlas` — ATLAS Alignment File

Stores a DTW (dynamic time warping) warp path mapping source timestamps to target
timestamps for a variant (e.g., an extended cut or alternate language dub).

```text
┌────────────────────────────────────────────────────────┐
│  ATLAS Magic: "ATLS" (4 bytes)                         │
│  Source ID: [u8; 8]   (e.g., track ID)                 │
│  Target ID: [u8; 8]   (e.g., variant ID)               │
│  Point count: u32                                      │
├────────────────────────────────────────────────────────┤
│  WarpPoint[0]  { source_t: f32, target_t: f32 }        │
│  WarpPoint[1]  { ... }                                 │
│  WarpPoint[N]  { ... }                                 │
└────────────────────────────────────────────────────────┘
```

---

## Part V — Compilation Exclusions

Files and folders the compiler always skips:

| Path                  | Reason                                                  |
| --------------------- | ------------------------------------------------------- |
| `configs/`            | Toolchain config — never compiled, never history-tracked|
| `.history/`           | History store — read by the compiler CLI, not compiled  |
| `artwork/`            | Binary image assets — not compiled (only URLs in .aura) |
| `motion/`             | Binary video assets — not compiled (only URLs in .aura) |
| `trailers/`           | Binary video assets — not compiled (only URLs in .aura) |
| `stems/`              | Audio stems — not compiled                              |
| `dist/`               | Compiler output folder — never re-compiled              |
| Paths in `ignore.aura`| Per-project exclusion list                              |

Art, motion, and trailer assets are uploaded separately to the cloud store to obtain their
URL. That URL is stored as literal text in `info/arts.aura`. No binary media files are
compiled or bundled into `.atom` or `.hami` outputs.

---

## Part VI — ID Prefix Registry (`core/src/id.rs`)

Every generated ID has a type prefix. The prefix encodes the object class.

| Prefix | Class        | Struct             | Example ID   |
| ------ | ------------ | ------------------ | ------------ |
| `t`    | track        | AtomNode           | `t7xab3c`    |
| `c`    | collection   | HamiNode           | `c8xab3d`    |
| `p`    | person       | PersonNode         | `p4xt9k2`    |
| `v`    | variant      | AtomNode           | `v3qr7st`    |
| `ep`   | episode      | AtomNode           | `ep7xb3n`    |
| `sn`   | season       | HamiNode           | `sn2kr9l`    |
| `tv`   | series       | HamiNode           | `tv4x7ab`    |
| `f`    | film         | HamiNode           | `f6np2qr`    |
| `dc`   | documentary  | HamiNode           | `dc3wr8x`    |
| `pc`   | podcast      | HamiNode           | `pc5xk4m`    |
| `an`   | animation    | HamiNode           | `an9vl3b`    |
| `sp`   | speech       | AtomNode           | `sp2xr7n`    |
| `b`    | audiobook    | AtomNode           | `b8mt4kx`    |
| `mv`   | music video  | HamiNode           | `mv6xp3l`    |
| `sg`   | single       | HamiNode           | `sg4xr9t`    |
| `cy`   | interview    | AtomNode           | `cy3wp8n`    |
| `r`    | rights       | HamiNode           | `r1xb7kp`    |
| `i`    | info doc     | HamiNode           | `i0xmt3q`    |
| `tx`   | take         | TakeObject         | `tx3ab7k`    |
| `st`   | studio       | StudioNode         | `st4xab3c`   |
| `lb`   | label        | LabelNode          | `lb7mn4rp`   |
| `ar`   | art          | ArtNode            | `ar4xab3c`   |
| `mo`   | motion       | MotionNode         | `mo7xk9p2`   |
| `tr`   | trailer      | TrailerNode        | `tr6xp3lm`   |

ID format: `{prefix}{6 alphanumeric chars}` — charset `a-z0-9`, 36^6 = 2,176,782,336 values
per prefix. The generator checks each candidate against the active project registry before
returning it. IDs are never hand-authored.

---

## Part VII — Namespace Resolution Order

When the compiler encounters an `@domain/id` reference it resolves it in this order:

```text
1. In-file symbol table
   (nodes defined in the current .aura file)

2. Project-local info/ and meta/ symbol tables
   (info/people.aura, info/arts.aura, info/studios.aura, etc.)

3. Project-local tracks/, episodes/, scenes/ registry
   (from namespace.aura files in each sub-folder)

4. Project-level catalog registry
   (the root namespace.aura exports:: block)

5. Global cloud registry
   (via @aduki.org/domain/id lookup — requires network)

6. Unresolved → forward arc
   (stored as a dangling reference; warning unless directives::strict -> live)
```

Local resolution always wins. The compiler never makes a network call unless all local
tables have been exhausted.

---

*Compiler Structure Reference — v0.1*
*Workspace layout: core / compiler / engine*
*Pipeline: lexer → parser → namespace loader → resolver → time normalizer → emitter*
*Output formats: .atom (interval tree) · .hami (B-Tree manifest) · .atlas (DTW alignment)*
