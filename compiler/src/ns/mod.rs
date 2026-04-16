//! Namespace module — project symbol discovery and export resolution.
//!
//! # Sub-modules
//!
//! | Module   | Responsibility                                           |
//! | -------- | -------------------------------------------------------- |
//! | `load`   | Reads `namespace.aura` files; builds the symbol table    |
//! | `export` | Resolves the `exports::` block in the root manifest      |
//!
//! # Module access paths
//!
//! ```
//! compiler::ns::load::NamespaceLoader
//! compiler::ns::load::Manifest
//! compiler::ns::load::Entry
//! compiler::ns::export::ExportResolver
//! compiler::ns::export::Export
//! compiler::ns::export::ExportPath
//! ```

pub mod export;
pub mod load;

pub use export::{Export, ExportPath, ExportResolver};
pub use load::{Entry, Manifest, NamespaceLoader};
