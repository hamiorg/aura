//! Two-phase `@domain/id` reference resolver.
//!
//! # Phase 1 — Symbol table build
//!
//! Walks the parsed AST of the current file plus its `info/` and `meta/`
//! dependencies to build a local symbol table: all defined nodes and IDs.
//!
//! # Phase 2 — Reference resolution
//!
//! Resolves every `@domain/id` reference against the following cascade,
//! stopping at the first match:
//!
//! 1. In-file symbol table
//! 2. Project `info/` and `meta/` tables
//! 3. Project `tracks/`, `episodes/`, `scenes/` registries
//! 4. Project catalog registry (`exports::` block)
//! 5. Global cloud registry (`@aduki.org/…`) — network required
//! 6. Unresolved → forward arc (warning, or error if `strict` is set)
//!
//! # Forward arcs
//!
//! Unresolved references are stored as `ForwardArc` values and emitted
//! as warnings unless `directives::strict -> live` is set, in which case
//! they are compile errors.

use crate::error::{CompileError, Diagnostic};
use crate::parse::ast::{Document, RefBody, Reference};
use std::collections::HashMap;

/// Resolution status of a reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
  /// Resolved to a local or catalog symbol.
  Local(String),
  /// Resolved to a global cloud URI.
  Global(String),
  /// Not yet resolved — stored as a forward arc.
  Forward,
}

/// A resolved or pending reference entry.
#[derive(Debug, Clone)]
pub struct Resolved {
  /// The full original reference text, e.g. `"@person/p4xt9k2"`.
  pub raw: String,
  /// Resolution status.
  pub status: Status,
}

/// An unresolved reference stored as a forward arc.
#[derive(Debug, Clone)]
pub struct ForwardArc {
  /// Full reference text, e.g. `"@track/t7xab3c"`.
  pub reference: String,
  /// Source file path.
  pub file: std::path::PathBuf,
  /// 1-indexed line where the reference appears.
  pub line: u32,
}

/// The project-wide symbol table built in Phase 1.
#[derive(Debug, Default)]
pub struct SymbolTable {
  /// Maps a fully-qualified symbol path to its owning file.
  /// E.g. `"person/p4xt9k2"` → `"info/people.aura"`.
  symbols: HashMap<String, std::path::PathBuf>,
}

impl SymbolTable {
  pub fn new() -> Self {
    Self::default()
  }

  /// Registers a symbol produced by Phase 1 scanning.
  pub fn insert(&mut self, key: impl Into<String>, file: std::path::PathBuf) {
    self.symbols.insert(key.into(), file);
  }

  /// Returns `true` if a symbol with this key exists.
  pub fn contains(&self, key: &str) -> bool {
    self.symbols.contains_key(key)
  }

  /// Returns the file that defines this symbol, if known.
  pub fn owner(&self, key: &str) -> Option<&std::path::PathBuf> {
    self.symbols.get(key)
  }

  pub fn len(&self) -> usize {
    self.symbols.len()
  }

  pub fn is_empty(&self) -> bool {
    self.symbols.is_empty()
  }
}

/// The two-phase reference resolver.
pub struct Resolver {
  /// Phase 1 symbol table (built from the AST scan).
  pub table: SymbolTable,
  /// Forward arcs collected during Phase 2.
  pub forward: Vec<ForwardArc>,
  /// Unresolvable-reference warnings or errors accumulated.
  pub diagnostics: Vec<Diagnostic>,
  /// Whether unresolved references are hard errors.
  strict: bool,
}

impl Resolver {
  pub fn new(strict: bool) -> Self {
    Self {
      table: SymbolTable::new(),
      forward: Vec::new(),
      diagnostics: Vec::new(),
      strict,
    }
  }

  // ---------------------------------------------------------------- //
  // Phase 1 — symbol table population

  /// Registers all defined IDs and node paths from a parsed document.
  pub fn register_document(&mut self, doc: &Document<'_>, file: std::path::PathBuf) {
    for ns in &doc.namespaces {
      if !ns.path.is_empty() {
        self.table.insert(ns.path.clone(), file.clone());
      }
    }
  }

  // ---------------------------------------------------------------- //
  // Phase 2 — reference resolution

  /// Attempts to resolve a single reference against the symbol table.
  ///
  /// Returns the resolution status. Stores a forward arc if the
  /// reference cannot be resolved locally.
  pub fn resolve(&mut self, reference: &Reference<'_>, source_file: &std::path::Path) -> Status {
    let key = build_key(reference);

    if self.table.contains(&key) {
      return Status::Local(key);
    }

    // Global cloud reference — leave as-is for the engine to resolve.
    if reference.domain == "aduki.org" {
      if let RefBody::Global(uri) = &reference.body {
        return Status::Global(uri.to_string());
      }
    }

    // Unresolved — store as a forward arc.
    let raw = format!("@{}/{}", reference.domain, key_body(reference));
    if self.strict {
      self.diagnostics.push(
        Diagnostic::error(format!("unresolved reference `{}`", raw))
          .with_file(source_file.to_path_buf())
          .with_span(reference.span),
      );
    } else {
      self.diagnostics.push(
        Diagnostic::warning(format!("unresolved reference `{}` (forward arc)", raw))
          .with_file(source_file.to_path_buf())
          .with_span(reference.span),
      );
      self.forward.push(ForwardArc {
        reference: raw,
        file: source_file.to_path_buf(),
        line: reference.span.line,
      });
    }

    Status::Forward
  }

  /// Returns `true` if any hard errors were accumulated.
  pub fn has_errors(&self) -> bool {
    self
      .diagnostics
      .iter()
      .any(|d| d.level == crate::error::Level::Error)
  }

  /// Converts accumulated errors into a `CompileError`.
  pub fn into_error(self) -> Option<CompileError> {
    if self.diagnostics.is_empty() {
      None
    } else {
      Some(CompileError {
        diagnostics: self.diagnostics,
      })
    }
  }
}

// -------------------------------------------------------------------- //
// Helpers

fn build_key(r: &Reference<'_>) -> String {
  format!("{}/{}", r.domain, key_body(r))
}

fn key_body(r: &Reference<'_>) -> String {
  match &r.body {
    RefBody::Single(id) => id.to_string(),
    RefBody::List(ids) => ids.join(","),
    RefBody::Path(parts) => parts.join("/"),
    RefBody::Global(uri) => uri.to_string(),
  }
}
