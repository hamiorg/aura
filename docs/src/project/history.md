# AURA History System (`history`)

> **Source-of-truth, AST-node-based version control for the AURA ecosystem.**

---

## 1. Core Philosophy

The AURA history system provides permanent, append-only provenance for media metadata.

- **Source of Truth:** It tracks changes *only* to the human-authored `.aura` text files. It
  **never** stores compiled `.atom` or `.hami` binaries in the ledger.
- **Node-Level Diffs:** Because AURA relies on slash-identifiers (e.g., `verse/one/line/two`),
  the diffing algorithm tracks changes at the **AST node level**, not raw text lines.
- **Append-Only Immutability:** Once a take is recorded, it cannot be modified or deleted.
  History grows forward. The ledger is a permanent record.

### What History Tracks

| Tracked by `.history/`        | Not tracked                               |
| ----------------------------- | ----------------------------------------- |
| `.aura` source files          | `.atom` compiled binaries                 |
| `info/` `.aura` files         | `.hami` compiled binaries                 |
| `meta/` `.aura` files         | `.atlas` DTW alignment files              |
| Content node text changes     | `configs/` folder (excluded by design)    |
| Credit and rights edits       | Binary assets (artwork, stems, video)     |
| Annotator attribution edits   | Art, motion, and trailer cloud links      |

Art, motion, and trailer URLs stored in `info/arts.aura` **are** tracked (they are text
values in `.aura` files), but the remote media assets they point to are not.

---

## 2. The Isolated Store

Version control data is isolated from the working directory, under `.history/` at project
root. History objects are serialized natively as `.hami` files.

```text
project-root/
├── .history/
│   ├── objects/               <- Immutable take objects (AST SourceDeltas)
│   │   ├── tx3ab7k.hami
│   │   └── tx9zz1p.hami
│   ├── marks/
│   │   ├── v1.0               <- plain text: take ID "tx3ab8m"
│   │   └── premiere           <- plain text: take ID "tx3ab7k"
│   ├── streams/               <- Pointers to latest takes per stream
│   │   ├── main               <- head of the main stream (plain text: take ID)
│   │   └── translation-fr     <- head of a parallel stream
│   └── HEAD                   <- Active stream pointer (plain text: stream name)
├── info/
├── meta/
├── tracks/
├── configs/                   <- NOT tracked by history
└── dist/
```

The `.history/` folder is never compiled and never published to the cloud store. When using
`aura release`, the published artifact is the compiled `.atom` / `.hami` — not the history
store.

> **Note:** `aura dub` creates an independent full-history copy of the project, including the
> entire `.history/` folder. This is the mechanism for creating forks with full provenance.

---

## 3. Data Structures (`core`)

### TakeObject

```rust
pub struct TakeObject {
    pub id: String,                  // e.g., "tx3ab7k" — tx prefix, 6 alphanumeric chars
    pub parent: Option<String>,   // previous take (None for origin take)
    pub stream: String,              // stream name this take belongs to (default: "main")
    pub message: Option<String>,     // optional human-readable description
    pub timestamp: u64,              // Unix timestamp of the take
    pub deltas: Vec<SourceDelta>,    // AST node text diffs relative to parent
}
```

### SourceDelta

```rust
pub enum SourceDelta {
    /// Upsert a node (add if new, replace if existing)
    UpsertNode {
        path: String,       // slash-identifier path, e.g., "verse/one/line/two"
        aura: String,  // full AURA text block for this node
    },
    /// Remove a node entirely
    DropNode {
        path: String,       // slash-identifier path of the removed node
    },
}
```

### StreamPointer

```rust
pub struct StreamPointer {
    pub stream: String,  // e.g., "main", "translation-fr"
    pub head: String, // ID of the most recent take on this stream
}
```

### MarkEntry

```rust
pub struct MarkEntry {
    pub name: String,    // e.g., "v1.0", "premiere", "final-mix"
    pub take: String, // the take this mark labels
}
```

---

## 4. The Delta Chain

Each take stores only the **difference** from its parent take. To reconstruct the full source
state at any take, the engine replays the chain from the origin take to the target:

```text
origin (tx3ab7k)
  └── delta: UpsertNode { verse/one/line/one, "The signal..." }

tx3ab8m (parent: tx3ab7k)
  └── delta: UpsertNode { verse/one/line/two, "She said..." }

tx3ab9n (parent: tx3ab8m)
  └── delta: UpsertNode { chorus/one/line/one, "Find me..." }
           + DropNode   { bridge/one }
```

Reconstructing `tx3ab9n` replays all three deltas in sequence — producing a virtual in-memory
`.aura` document that matches that take state without writing a file to disk.

---

## 5. Compilation Workflows

| Command                          | Description                                                      |
| -------------------------------- | ---------------------------------------------------------------- |
| `aura compile`                   | Ignore `.history/` entirely. Compile working draft only.         |
| `aura compile --take tx3ab7k`    | Replay delta chain to reconstruct + compile historical state     |
| `aura compile --embed-history`   | Embed HistoryNodes (class `0x14`) in `.atom` for runtime queries |

---

## 6. Toolchain Reference

### Taking Snapshots

| Command              | Description                                          |
| -------------------- | ---------------------------------------------------- |
| `aura take`          | Record current draft as a new immutable take         |
| `aura take "msg"`    | Record a take with a descriptive message             |
| `aura mark name`     | Attach a human-readable name to the current take     |

### Navigating History

| Command                    | Description                                          |
| -------------------------- | ---------------------------------------------------- |
| `aura rewind take-id`      | Restore draft to a specific take by ID               |
| `aura rewind mark-name`    | Restore draft to a named mark                        |
| `aura rewind ~n`           | Restore draft n takes before current                 |
| `aura ledger`              | Show the full take history of this project           |
| `aura ledger node/path`    | Show the history of a specific node                  |
| `aura delta take-a take-b` | Show changed nodes between two takes                 |

### Streams

| Command                    | Description                                          |
| -------------------------- | ---------------------------------------------------- |
| `aura stream open name`    | Create a new named development stream                |
| `aura stream close name`   | Close and archive a named stream                     |
| `aura stream list`         | List all open streams                                |
| `aura mix stream-name`     | Mix a completed stream into the current stream       |

### Working State

| Command              | Description                                            |
| -------------------- | ------------------------------------------------------ |
| `aura hold`          | Park the current working draft without taking          |
| `aura hold restore`  | Restore a previously parked draft                      |
| `aura release`       | Publish the current take to `@aduki.org` distribution  |
| `aura sync`          | Pull the latest released state from `@aduki.org`       |
| `aura dub`           | Create an independent full-history copy of the project |

---

## 7. History Constraints

- Tracks only `.aura` source files. Compiled artifacts are always reproduced deterministically.
- The `configs/` folder is excluded by design. Credentials are never versioned.
- The `.history/` folder itself is excluded from compiled output except in `--embed-history`.
- History resolution is **always read-only**. The working state changes only via `take`,
  `rewind`, or `mix`.

---

*AURA History System (history) — v0.1*
*Append-only, AST-node-based version control for `.aura` source files.*
*No compiled artifacts. No credentials. No binary assets.*
