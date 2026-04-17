//! Emitter module — binary output generators for `.hami`, `.atom`, `.atlas`.
//!
//! # Sub-modules
//!
//! | Module  | Output  | Description                                  |
//! | ------- | ------- | -------------------------------------------- |
//! | `hami`  | `.hami` | B-Tree manifest (credits, vocab, metadata)   |
//! | `atom`  | `.atom` | Flat-array augmented interval tree           |
//! | `atlas` | `.atlas`| DTW warp path for variant stream alignment   |
//!
//! # Module access paths
//!
//! ```ignore
//! compiler::emit::hami::HamiEmitter
//! compiler::emit::atom::AtomEmitter
//! compiler::emit::atlas::AtlasEmitter
//! compiler::emit::atlas::AlignSpec
//! ```

pub mod atlas;
pub mod atom;
pub mod hami;

pub use atlas::{AlignSpec, AtlasEmitter};
pub use atom::AtomEmitter;
pub use hami::HamiEmitter;
