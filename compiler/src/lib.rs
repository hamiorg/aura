//! AURA Compiler — public API.
//!
//! This crate compiles `.aura` source files into `.atom`, `.hami`, and
//! `.atlas` binary artifacts. It is one half of the AURA toolchain
//! (the other half being the engine, which is a separate crate).
//!
//! # Pipeline
//!
//! ```text
//! .aura source
//!     |
//!  lex::scan      -> token stream (zero-copy &str slices)
//!     |
//!  parse::ast     -> typed AST
//!  parse::time    -> time expression normalization
//!  parse::resolve -> @domain/id reference resolution
//!  parse::inherit -> >> arc expansion
//!     |
//!  emit::hami     -> .hami B-Tree manifest
//!  emit::atom     -> .atom flat-array interval tree
//!  emit::atlas    -> .atlas DTW warp path
//! ```
//!
//! # Module access paths
//!
//! ```
//! compiler::lex::scan::Scanner
//! compiler::parse::ast::Document
//! compiler::emit::atom::AtomEmitter
//! compiler::hist::store::HistoryStore
//! compiler::cfg::load::ConfigLoader
//! compiler::error::CompileError
//! ```

pub mod cfg;
pub mod cmd;
pub mod directives;
pub mod emit;
pub mod error;
pub mod hist;
pub mod lex;
pub mod lint;
pub mod ns;
pub mod parse;

pub use error::{CompileError, Diagnostic, Level, Result, Span};
