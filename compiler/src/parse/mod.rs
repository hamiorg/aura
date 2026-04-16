//! Parser module — AST construction, time normalization, reference
//! resolution, and `>>` arc expansion.
//!
//! # Pipeline position
//!
//! ```text
//! lex::scan → [token stream] → parse → [resolved AST] → emit
//! ```
//!
//! # Sub-modules
//!
//! | Module      | Responsibility                                      |
//! | ----------- | --------------------------------------------------- |
//! | `ast`       | AST node types (`Document`, `Namespace`, `Field`)   |
//! | `time`      | Time expression normalizer → `Interval`             |
//! | `resolve`   | Two-phase `@domain/id` reference resolver           |
//! | `inherit`   | `>>` arc expander (parent fields merged into child) |
//!
//! # Module access paths
//!
//! ```
//! compiler::parse::ast::Document
//! compiler::parse::ast::Namespace
//! compiler::parse::ast::Field
//! compiler::parse::ast::Value
//! compiler::parse::ast::NodeType
//! compiler::parse::time::TimeNorm
//! compiler::parse::time::parse_seconds
//! compiler::parse::resolve::Resolver
//! compiler::parse::resolve::SymbolTable
//! compiler::parse::inherit::InheritExpander
//! ```

pub mod ast;
pub mod inherit;
pub mod resolve;
pub mod time;

pub use ast::{
  Child, Document, Field, FieldMarker, Namespace, NodeType, RefBody, Reference, TimeExpr, Value,
};
pub use inherit::InheritExpander;
pub use resolve::{ForwardArc, Resolver, Status, SymbolTable};
pub use time::{parse_seconds, TimeNorm};
