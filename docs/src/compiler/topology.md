# Polyglot Persistence and Data Topology

The Traverse protocol splits data responsibility across three physically
distinct execution targets. Understanding where each query must be routed
is the single most important architectural concept for integrators.

---

## The Three Execution Targets

| Target | Role | Process boundary |
| ------ | ---- | ---------------- |
| **Kùzu Graph Engine** | Structural resolution: byte offsets, access validation, relationship traversal | Cloud edge node (embedded Rust) |
| **ScyllaDB Blob Store** | Immutable binary artifact storage: `.atom`, `.hami`, `.atlas` payloads | Global distributed cluster |
| **Local AURA Engine** | Temporal stabbing queries, SIMD interval evaluation, OCPN marking vector | Client RAM via `mmap` |

The client SDK (< 2 MB budget) contains **only** the local engine. It never
embeds Kùzu and never contacts ScyllaDB directly.

---

## Query Routing Matrix

Use this matrix as the authoritative decision table. Route every query to
**exactly one** target based on intent.

| Query intent | Target | Justification |
| ------------ | ------ | ------------- |
| Verify cross-catalog access rights | Kùzu (edge node) | Evaluates explicit licensing arcs via `RightsNode` relationships before resolving any `::` pointers. Operates on graph logic only — never carries payload data. |
| Resolve memory offsets for media | Kùzu (edge node) | Calculates exact byte offsets into the `.atom` / `.hami` binaries without fetching the file payload. Acts as a traffic director, not a data store. |
| Retrieve raw binary artifacts | ScyllaDB | Fetches the immutable `.atom`, `.hami`, or `.atlas` arrays. Shard routing is instantaneous via content-addressed ID hash. No inter-node coordination. |
| Temporal stabbing query (`t = currentTime`) | Local engine (RAM `mmap`) | Executes AVX-2 SIMD branch pruning. Finds all intervals `[low, high]` overlapping `t` in < 0.1 ms across 100 000+ sync points. |
| Filter by node class | Local engine (RAM `mmap`) | Applies `node_class` bitmask inside the SIMD loop. Identifies mood, translation, lyrics, or access nodes in a single pass at no additional per-node cost. |
| Reconstruct historical provenance | AURA toolchain (`.history/`) | Replays `SourceDelta` chain to generate a virtual in-memory AST for a specific `take` ID. Never touches the hot path. |
| Access weight comparison (ReBAC) | Local engine (RAM `mmap`) | `caller_weight >= node.access_weight` — a single branchless integer comparison packed into the upper 16 bits of `node_class`. No graph traversal at runtime. |

---

## Ingestion Fork — Payload Lifecycle

When `aura release` is executed, the compiler splits one `.aura` source
file into two independent artifact streams that diverge at the edge node.

```text
  .aura source file
       │
       ▼
  ┌─────────────────────────────────────────┐
  │  aura compile (toolchain)               │
  │                                         │
  │  Parser → Resolver → Linter             │
  │       │                                 │
  │  ┌────▼────────────┐                    │
  │  │ HamiEmitter     │  ← manifests,      │
  │  │                 │    schema, credits  │
  │  └────────┬────────┘                    │
  │           │  .hami                      │
  │  ┌────────▼────────┐                    │
  │  │ AtomEmitter     │  ← interval nodes  │
  │  │ + AccessWeights │    packed weights   │
  │  └────────┬────────┘                    │
  │           │  .atom                      │
  └───────────┼─────────────────────────────┘
              │
        aura release
              │
  ┌───────────┼─────────────────────────────┐
  │           │  Ingestion fork             │
  │    ┌──────▼──────┐   ┌───────────────┐  │
  │    │  ScyllaDB   │   │  Kùzu graph   │  │
  │    │  blob store │   │  (edge node)  │  │
  │    │             │   │               │  │
  │    │  .atom blobs│   │  struct edges │  │
  │    │  .hami blobs│   │  byte offsets │  │
  │    │  .atlas blobs   │  access arcs  │  │
  │    └──────┬──────┘   └───────┬───────┘  │
  └───────────┼──────────────────┼──────────┘
              │                  │
        QUIC/HTTP3 request       │
              │◄─────────────────┘
              │  Kùzu resolves offsets,
              │  validates access weight,
              │  returns ScyllaDB content-hash
              │
        Client receives
        ┌─────▼──────────────────────────────┐
        │  .atom mapped into RAM via mmap2   │
        │  AtomNode[] cast zero-copy         │
        │  AVX-2 SIMD stabbing queries       │
        │  < 0.1 ms per frame at 60 fps      │
        └────────────────────────────────────┘
```

---

## Anti-Patterns

The following patterns are explicitly prohibited. Each violates the
separation of concerns and either creates security vulnerabilities or
destroys the performance guarantees.

| Anti-pattern | Why it is wrong | Correct approach |
| ------------ | --------------- | ---------------- |
| **Querying Kùzu for payload text** (e.g. lyrics, translations) | Corrupts separation of concerns. Kùzu holds structural offsets, not content. Embedding payloads in the graph collapses edge node throughput. | Retrieve payload data from the `mmap` zero-copy region via the local engine. |
| **Client-side access validation** (replicating Kùzu logic in the SDK) | Creates severe security vulnerabilities. The client SDK operates without authenticated session context. Rights checking cannot be delegated to untrusted callers. | All access validation must be performed by the cloud edge node using Kùzu's authenticated graph context before binaries are served. |
| **Bypassing the cache hierarchy** (polling ScyllaDB origin directly for freshness) | Injects inter-node latency into the 16.6 ms playback hot path. At 60 fps, even a 2 ms extra round-trip causes dropped frames. | Rely on the L2 CDN edge, which validates cache freshness via a HEAD request against the content-addressed ID hash — a single integer comparison, not a payload fetch. |
| **Storing mutable state in `.atom` / `.hami`** | Binary artifacts are immutable once written to ScyllaDB. Any mutation requires a new compile + release cycle. Writing mutable data into artifact fields causes stale reads on cache-miss. | Use the Kùzu graph layer for mutable relationship state (e.g. access tier changes). The artifact layer is always append-only. |
| **Running `aura compile` without `aura sanitize` on untrusted input** | The zero-copy lexer cannot process backslash escape sequences. An unsanitized file triggers the error path in the AVX-2 SIMD loop, halting compilation with an unhelpful error. | Run `aura sanitize` first, or configure the IDE extension with the local LLM bridge for real-time normalization before save. |

---

## Human-to-Machine Bridge

The AURA authoring experience is deliberately decoupled from the lexer's
mathematical constraints via a three-layer bridge:

### Layer 1 — `aura sanitize` (command-line pre-pass)

Runs before `aura compile`. Scans for forbidden byte sequences (`\"`, `\'`,
literal `\n`) and replaces them with Unicode equivalents that the zero-copy
lexer treats as plain content bytes. The transformation is an O(n) single
forward pass — safe to run on every save in CI.

```sh
aura sanitize                # normalize all files in project
aura sanitize --dry-run      # preview changes without writing
aura sanitize --path tracks/t7xab3c.aura  # single file
```

### Layer 2 — IDE LLM Integration (`configs/llm.aura`)

The `configs/llm.aura` file declares a locally hosted or API-backed LLM
endpoint. IDE extensions (VS Code, Zed, Neovim) send the surrounding ±5
lines of context to the LLM when a forbidden character is detected. The
LLM returns a corrected AURA fragment that the extension replaces inline
before the file is saved. The lexer never sees the forbidden sequence.

Because the output space is tightly constrained (the model only needs to
produce valid AURA syntax, not general code), constrained LLMs achieve
near-100% accuracy on this specific task with a minimal 1–7 B parameter
model running locally via Ollama.

### Layer 3 — Documented syntactic equivalencies

The AURA language provides clean Unicode alternatives for every common
authoring edge case:

| Human input | AURA-safe equivalent | Code point |
| ----------- | -------------------- | ---------- |
| `\"text\"`  | `"text"`             | U+201C / U+201D |
| `it's`      | `it\u{2019}s` (or `'`) | U+2019   |
| `--`        | `—`                  | U+2014     |
| `...`       | `…`                  | U+2026     |

Because all bytes `≥ 0x20` are valid content bytes in the AURA lexer, any
Unicode character other than the four structural sigils (`:`, `-`, `@`, `#`)
is a safe content byte when used outside a structural position.

---

*AURA Compiler Reference — Polyglot Topology*
*v0.3.3-beta.1*
