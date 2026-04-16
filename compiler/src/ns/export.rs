//! `exports::` block resolver.
//!
//! The root `namespace.aura` contains an `exports::` block that re-exports
//! sub-namespaces for the compiler to discover:
//!
//! ```aura
//! exports::
//!   info       -> @info/metadata
//!   people     -> @info/people
//!   tracks     -> @tracks/*
//!   collection -> c8xab3d.aura
//! ```
//!
//! The `ExportResolver` reads this block and returns a list of paths the
//! compiler should visit when building the project symbol table.

use crate::error::Result;
use std::path::PathBuf;

/// A single export entry from the `exports::` block.
#[derive(Debug, Clone)]
pub struct Export {
  /// The export alias, e.g. `"info"`, `"people"`, `"tracks"`.
  pub alias: String,
  /// The resolved file or folder path relative to the project root.
  /// A `*` wildcard means "all files in this folder".
  pub path: ExportPath,
}

/// The path target of an export entry.
#[derive(Debug, Clone)]
pub enum ExportPath {
  /// A single file, e.g. `"c8xab3d.aura"`.
  File(PathBuf),
  /// All `.aura` files in a folder, e.g. from `@tracks/*`.
  Glob(PathBuf),
  /// An `@info/` or `@meta/` file reference.
  Info { folder: String, file: String },
}

/// Resolves the `exports::` block in a root `namespace.aura`.
pub struct ExportResolver {
  /// Project root directory.
  root: PathBuf,
}

impl ExportResolver {
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self { root: root.into() }
  }

  /// Parses and resolves all exports from the given `namespace.aura` text.
  pub fn resolve(&self, text: &str) -> Result<Vec<Export>> {
    let mut exports = Vec::new();
    let mut in_exports = false;

    for line in text.lines() {
      let trimmed = line.trim();

      if trimmed == "exports::" {
        in_exports = true;
        continue;
      }
      // Another top-level block ends the exports section.
      if !trimmed.is_empty() && trimmed.ends_with("::") && !line.starts_with(' ') {
        in_exports = false;
      }

      if in_exports {
        if let Some((alias, target)) = parse_export_line(trimmed) {
          let path = self.resolve_target(target)?;
          exports.push(Export {
            alias: alias.to_string(),
            path,
          });
        }
      }
    }

    Ok(exports)
  }

  fn resolve_target(&self, target: &str) -> Result<ExportPath> {
    // `@info/metadata` or `@meta/genres`
    if let Some(ref_body) = target.strip_prefix('@') {
      if let Some(slash) = ref_body.find('/') {
        let folder = &ref_body[..slash];
        let file = &ref_body[slash + 1..];
        if file == "*" {
          return Ok(ExportPath::Glob(self.root.join(folder)));
        }
        return Ok(ExportPath::Info {
          folder: folder.to_string(),
          file: file.to_string(),
        });
      }
    }

    // `@tracks/*` — glob
    if target.ends_with("/*") {
      let folder = target.trim_start_matches('@').trim_end_matches("/*");
      return Ok(ExportPath::Glob(self.root.join(folder)));
    }

    // Plain file reference, e.g. `c8xab3d.aura`
    Ok(ExportPath::File(self.root.join(target)))
  }
}

fn parse_export_line(line: &str) -> Option<(&str, &str)> {
  let arrow = line.find("->")?;
  let alias = line[..arrow].trim();
  let target = line[arrow + 2..].trim();
  if alias.is_empty() || target.is_empty() {
    return None;
  }
  Some((alias, target))
}
