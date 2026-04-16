//! `>>` inherits-from arc expander.
//!
//! AURA allows any content file to inherit fields from a base schema or
//! template via the `>>` sigil:
//!
//! ```aura
//! >> @info/metadata
//! ```
//!
//! This instructs the compiler to merge all fields from the referenced
//! document (or namespace) into the current AST node before resolving
//! references and emitting binary output.
//!
//! # Rules
//!
//! 1. The inherited file is read and parsed first (if not already in the
//!    project symbol table).
//! 2. Fields in the current file override inherited fields. The inheriting
//!    file always wins.
//! 3. Inheriting is shallow: only the direct fields of the referenced
//!    namespace are merged, not its nested sub-namespaces.
//! 4. Circular inheritance is detected and reported as a hard error.

use crate::error::{CompileError, Result};
use crate::parse::ast::{Child, Document, Namespace, Value};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// The inherit expander resolves `>>` arcs in the AST.
pub struct InheritExpander {
  /// Tracks which files are currently being expanded to detect cycles.
  expanding: HashSet<PathBuf>,
}

impl InheritExpander {
  pub fn new() -> Self {
    Self {
      expanding: HashSet::new(),
    }
  }

  /// Expands all `>>` arcs in the given document in place.
  ///
  /// `load` is a callback that loads and parses a referenced `.aura`
  /// file given its path relative to the project root. The expander
  /// calls it when it needs to read the inherited document.
  pub fn expand<F>(&mut self, doc: &mut Document<'_>, source_path: &Path, load: F) -> Result<()>
  where
    F: Fn(&Path) -> Result<Vec<(String, String)>>,
  {
    let canonical = source_path
      .canonicalize()
      .unwrap_or_else(|_| source_path.to_path_buf());

    if self.expanding.contains(&canonical) {
      return Err(CompileError::msg(format!(
        "circular inheritance detected in `{}`",
        source_path.display()
      )));
    }
    self.expanding.insert(canonical.clone());

    // Collect all top-level `>>` arcs and the namespaces to merge into.
    for ns in &mut doc.namespaces {
      self.expand_namespace(ns, source_path, &load)?;
    }

    self.expanding.remove(&canonical);
    Ok(())
  }

  fn expand_namespace<'a, F>(
    &mut self,
    ns: &mut Namespace<'a>,
    _source_path: &Path,
    load: &F,
  ) -> Result<()>
  where
    F: Fn(&Path) -> Result<Vec<(String, String)>>,
  {
    // Separate out any `>> @info/…` inherits arcs.
    let mut inherits_paths: Vec<String> = Vec::new();
    let mut remaining: Vec<Child<'a>> = Vec::new();

    for child in ns.children.drain(..) {
      match &child {
        Child::Field(f) => {
          if let Value::Inherits(r) = &f.value {
            // Build the file path from the reference domain/body.
            let path = reference_to_path(r);
            inherits_paths.push(path);
          } else {
            remaining.push(child);
          }
        }
        Child::Block(_) => remaining.push(child),
      }
    }

    // Merge inherited fields. Own fields override.
    let own_keys: HashSet<String> = remaining
      .iter()
      .filter_map(|c| {
        if let Child::Field(f) = c {
          Some(f.key.to_string())
        } else {
          None
        }
      })
      .collect();

    for inherit_path in inherits_paths {
      let rel = Path::new(&inherit_path);
      let inherited_fields = load(rel)?;
      for (key, _val) in inherited_fields {
        if !own_keys.contains(&key) {
          // The inherited field would be inserted here.
          // In a full implementation this would construct a
          // Field node from the parsed inherited document.
          // Left as a hook for the parser integration phase.
          let _ = key; // suppress unused warning in scaffold
        }
      }
    }

    ns.children = remaining;
    Ok(())
  }
}

impl Default for InheritExpander {
  fn default() -> Self {
    Self::new()
  }
}

fn reference_to_path(r: &crate::parse::ast::Reference<'_>) -> String {
  use crate::parse::ast::RefBody;
  match &r.body {
    RefBody::Single(id) => format!("{}/{}.aura", r.domain, id),
    RefBody::Path(parts) => format!("{}/{}.aura", r.domain, parts.join("/")),
    RefBody::Global(uri) => uri.to_string(),
    RefBody::List(_) => String::new(), // multi-ID inherits is invalid
  }
}
