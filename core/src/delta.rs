//! History delta chain types.
//!
//! These types are shared between the compiler (which writes `.history/`
//! objects) and the engine (which reads them for `--embed-history` mode).
//! They are defined in `core` so neither crate depends on the other.
//!
//! # Terminology (AURA-native, no git verbs)
//!
//! | AURA term | Meaning                                           |
//! | --------- | ------------------------------------------------- |
//! | take      | Immutable snapshot of the current document state |
//! | mark      | Human-readable name attached to a specific take  |
//! | stream    | Named parallel line of development               |
//! | delta     | Changes between any two takes                    |
//! | ledger    | Full ordered permanent history of all takes       |

/// An immutable take — a snapshot of one or more AURA source files at a
/// specific point in time. Takes are append-only; once recorded they can
/// never be modified or deleted.
#[derive(Debug, Clone)]
pub struct TakeObject {
  /// Unique take ID, e.g. `"tx3ab7k"` (`tx` prefix + 6 alphanum chars).
  pub id: String,
  /// ID of the parent take (`None` for the origin take).
  pub parent: Option<String>,
  /// Name of the stream this take belongs to (default: `"main"`).
  pub stream: String,
  /// Optional human-readable description of what changed.
  pub message: Option<String>,
  /// Unix timestamp when this take was recorded.
  pub timestamp: u64,
  /// AST node-level diffs relative to the parent take.
  pub deltas: Vec<SourceDelta>,
}

/// A node-level diff between two consecutive takes.
///
/// Because AURA uses slash-identifiers (`verse/one/line/two`), the diff
/// algorithm tracks changes at the AST node level, not raw text lines.
/// This makes the history immune to whitespace-only edits and keeps
/// delta chains small.
#[derive(Debug, Clone)]
pub enum SourceDelta {
  /// Add a new node or replace an existing one with updated AURA text.
  Upsert {
    /// Slash-identifier path, e.g. `"verse/one/line/two"`.
    path: String,
    /// Full AURA text block for this node (the new state).
    aura: String,
  },
  /// Remove a node entirely.
  Drop {
    /// Slash-identifier path of the removed node.
    path: String,
  },
}

/// Points to the most recent take on a named development stream.
#[derive(Debug, Clone)]
pub struct StreamPointer {
  /// Stream name, e.g. `"main"` or `"translation-fr"`.
  pub stream: String,
  /// ID of the most recent take on this stream.
  pub head: String,
}

/// A human-readable name pinned to a specific take.
///
/// Marks are written to `.history/marks/{name}` as plain text files
/// containing the take ID.
#[derive(Debug, Clone)]
pub struct MarkEntry {
  /// Mark name, e.g. `"v1.0"`, `"premiere"`, `"final-mix"`.
  pub name: String,
  /// The take this mark labels.
  pub take: String,
}
