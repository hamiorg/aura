# AURA Compiler Roadmap

> Planned and recently shipped features and architectural extensions.
> Items marked ✅ are implemented and shipped. Items without that mark
> are planned for a future release. Follow the changelog for release notes.

---

## R-1 — `aura sanitize` (Pre-Compiler Source Normalization)

**Status:** ✅ Implemented · Shipped: v0.3.3-beta.1

### Motivation

The zero-copy lexer operates under a strict constraint: no character escaping, no heap
allocation, no context-dependent parsing. This is what enables AVX-2 SIMD throughput in the
lexer hot path. However, human authors regularly type characters that are structurally
ambiguous (standard escaped quotes `\"`, directional quote pairs, etc.).

Today the author must manually avoid these. `aura sanitize` removes that burden.

### Design

`aura sanitize` is a standalone sub-command that runs **before** the primary compilation
pipeline and transforms the source file in-place:

```sh
aura sanitize [--dry-run] [--path <file>]
```

- Scans for forbidden byte sequences (primarily `\"` and ambiguous bracket pairs).
- Replaces them with unambiguous Unicode equivalents (`"` `"`, `«` `»`).
- Produces a normalized `.aura` file that the zero-copy lexer can ingest without exceptions.
- With `--dry-run`, only prints the proposed changes without writing.

The sanitizer will be integrated as an automatic pre-pass in `aura compile` behind a feature
flag so that CI pipelines can choose strict or lenient input modes.

---

## R-2 — `metaboolean.aura` (Extensible Boolean Literals)

**Status:** Implemented · Shipped: v0.3.3-beta.1

### Motivation

AURA currently ships two boolean literals: `live` (true) and `dark` (false). These are
media-native and expressive. However, different domains often have their own vocabulary for
boolean states — `cleared` / `blocked`, `published` / `draft`, `active` / `retired`.

Today the compiler maps every non-`live`/`dark` value through the parser's generic
string-value path. Authors must remember which values are boolean and which are arbitrary
strings. `metaboolean.aura` formalizes domain boolean extension.

### Design

A project may place a `metaboolean.aura` file in its `meta/` folder:

```aura
## FILE: meta/metaboolean.aura

booleans::

  cleared::
    true-maps-to  -> live
    false-maps-to -> dark

  blocked::
    true-maps-to  -> dark
    false-maps-to -> live

  published::
    true-maps-to  -> live
    false-maps-to -> dark
```

The compiler reads `meta/metaboolean.aura` before the parse phase. Any key declared here
is treated as a boolean field. Its value is normalized to `1` or `0` in the compiled `.hami`.
W004 (unknown boolean) linting is suppressed for declared keys.

Global platform-level booleans continue to ship in the standard vocabulary at
`@aduki.org/meta/booleans`.

---

## R-3 — `metaaccess.aura` (ReBAC Extensible Access — DAG Weights)

**Status:** Implemented · Shipped: v0.3.3-beta.1

### Motivation

The current `AccessLevel` enum has six fixed tiers (`open`, `archived`, `restricted`,
`gated`, `embargoed`, `locked`). This covers most publishing workflows, but streaming
platforms need finer-grained access (e.g. `press-only`, `premium-only`, `label-internal`).
Adding tiers today requires a compiler change.

### Design

`metaaccess.aura` declares a Directed Acyclic Graph (DAG) of access levels. The compiler
runs a topological sort on that DAG and emits integer weights into the `.atom` binary.

```aura
## FILE: meta/metaaccess.aura

access-dag::

  open::
    weight -> 1

  archived::
    weight  -> 2
    extends -> open

  restricted::
    weight  -> 3
    extends -> archived

  press-only::
    weight  -> 3
    extends -> archived

  gated::
    weight  -> 4
    extends -> restricted

  premium-only::
    weight  -> 4
    extends -> gated

  embargoed::
    weight  -> 5
    extends -> gated

  locked::
    weight  -> 6
    extends -> embargoed
```

At compile time, the compiler:

1. Reads `meta/metaaccess.aura` (falls back to the built-in six-tier enum if absent).
2. Topologically sorts the DAG to produce a deterministic integer weight for each tier.
3. Emits the weight as a `u16` into the `AccessNode` (`node_class` `0x13`).

At query time, the engine:

1. Reads the caller's access weight from the session token.
2. Compares `caller_weight >= node.access_weight` — a **single branchless integer instruction**.
3. No string comparison, no enum lookup, no heap allocation.

Custom tiers (`press-only`, `premium-only`, etc.) become first-class without any compiler
change.

---

## R-4 — `$` Vocab Escape Sigil (Keyword Collision Prevention)

**Status:** Implemented · Shipped: v0.3.3-beta.1

### Motivation

AURA vocabulary slugs sometimes collide with reserved keywords. For example, a genre slug
`live` collides with the boolean literal `live`. When a namespace block is named after such
a slug, the parser currently raises a W006 warning.

### Design

The `$` prefix on a namespace block name signals to the compiler that the name is a raw
vocabulary slug, not an AURA keyword:

```aura
vocab::

  $live::
    label -> "Live Recording"
    maps-to -> live

  $dark::
    label -> "Dark Ambient"
    maps-to -> dark
```

Rules:

- `$` is only valid as the first character of a namespace block name.
- W006 key-checking is disabled inside `$`-prefixed blocks.
- The compiled HAMI key strips the `$` prefix: `$live::` compiles to the `live` namespace.
- `$` is not valid inside key names or values — only namespace block openers.

---

## R-5 — Human-to-Machine Bridge (Constrained Generative Synthesis)

**Status:** Planned · Target: v0.4.x

### Motivation

AURA is designed for authors — people who think in nodes, not schemas. However, the zero-copy
lexer enforces a strict syntax that can frustrate authors working quickly in text editors
without AURA-aware tooling.

The Human-to-Machine Bridge makes the gap disappear at the IDE level.

### Design

IDE extensions (VS Code, Zed, Neovim) can integrate with the local LLM defined in
`configs/llm.aura`. When the extension detects a potentially invalid character sequence,
it:

1. Sends the surrounding context (±5 lines) to the local LLM endpoint.
2. The LLM returns the corrected AURA fragment.
3. The extension replaces the selection in-place — before the file is saved.

The compiled output is always valid AURA. The author never sees an error. The `aura
sanitize` pipeline (R-1) remains the fallback for files that bypass the IDE.

This is not a spell-checker. It is a real-time translation layer from human linguistic
expression to machine-parseable AURA, preserving meaning while normalizing syntax.

---

*AURA Roadmap — v0.3.2-beta.2*
*Planned items are subject to change. Follow the changelog for release notes.*
