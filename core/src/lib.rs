//! Core — shared zero-dependency data types for the AURA toolchain.
//!
//! Every `#[repr(C)]` struct lives here so the compiler and engine
//! see identical memory layouts. No business logic, no I/O.
//!
//! # Crate boundary rule
//! `core` has zero external dependencies. `compiler` and `engine` both
//! depend on `core` but never on each other.

pub mod access;
pub mod asset;
pub mod delta;
pub mod entity;
pub mod history;
pub mod id;
pub mod interval;
pub mod node;
pub mod person;
pub mod vocab;

// Convenience re-exports at the crate root.
pub use access::AccessLevel;
pub use asset::{ArtNode, MotionNode, TrailerNode};
pub use delta::{MarkEntry, SourceDelta, StreamPointer, TakeObject};
pub use entity::{LabelNode, StudioNode};
pub use history::HistoryNode;
pub use id::{IdGen, Prefix};
pub use interval::Interval;
pub use node::{AtomNode, HamiNode};
pub use person::{AnnotatorNode, AnnotatorRoles, PersonKind, PersonNode};
pub use vocab::{VocabKind, VocabNode};
