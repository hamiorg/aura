//! History subsystem — `.history/` object store, delta diffs, and replay.
//!
//! The history system provides permanent, append-only provenance for
//! AURA source files. It tracks only `.aura` text files — never compiled
//! binaries, binary assets, or the `configs/` folder.
//!
//! # AURA-native terminology (no git verbs)
//!
//! | Term     | Meaning                                               |
//! | -------- | ----------------------------------------------------- |
//! | take     | Immutable snapshot of the current document state     |
//! | mark     | Human-readable name attached to a specific take      |
//! | stream   | Named parallel line of development                   |
//! | delta    | Changes between any two takes (AST node level)       |
//! | ledger   | Full ordered permanent history of all takes           |
//! | rewind   | Restore draft to a previous take (non-destructive)   |
//! | mix      | Combine two streams into one                          |
//!
//! # Sub-modules
//!
//! | Module    | Responsibility                                           |
//! | --------- | -------------------------------------------------------- |
//! | `store`   | `.history/` object store reader/writer                   |
//! | `delta`   | AST node-level diff engine                               |
//! | `replay`  | Delta chain replayer → virtual source reconstruction      |
//!
//! # Module access paths
//!
//! ```
//! compiler::hist::store::HistoryStore
//! compiler::hist::delta::DeltaEngine
//! compiler::hist::replay::DeltaReplayer
//! ```

pub mod delta;
pub mod replay;
pub mod serial;
pub mod store;

pub use delta::DeltaEngine;
pub use replay::DeltaReplayer;
pub use store::HistoryStore;
