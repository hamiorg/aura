//! Namespace loader — reads `name.aura` files and builds the
//! project symbol table.
//!
//! Every AURA project has a `name.aura` at its root. Every
//! sub-folder (`info/`, `meta/`, `tracks/`, etc.) also has one.
//! Together they form the complete project index.
//!
//! # Root `name.aura`
//!
//! The compiler entry point. Contains the `name::` block with the
//! project ID, kind, slug, and lang, followed by an `exports::` block:
//!
//! ```aura
//! name::
//!   id        -> c8xab3d
//!   root      -> https://hami.aduki.org/aura/1.0
//!   kind      -> audio::album
//!   slug      -> signal-loss
//!   lang      -> en-US
//!
//! exports::
//!   info      -> @info/metadata
//!   people    -> @info/people
//!   tracks    -> @tracks/*
//! ```
//!
//! # Sub-folder `name.aura`
//!
//! ```aura
//! name::
//!   folder -> tracks
//!
//! contains::
//!   t7xab3c -> "Signal Loss"
//!   t4mn2rp -> "Fold"
//! ```

use crate::error::{CompileError, Result};
use crate::parse::resolve::SymbolTable;
use std::path::{Path, PathBuf};

/// A single entry in a sub-folder `name.aura` — an ID + label.
#[derive(Debug, Clone)]
pub struct Entry {
  /// Generated ID, e.g. `"t7xab3c"`.
  pub id: String,
  /// Human-readable label, e.g. `"Signal Loss"`. Optional.
  pub label: Option<String>,
}

/// The parsed contents of a `name.aura` file.
#[derive(Debug, Clone)]
pub struct Manifest {
  /// The folder this namespace describes (relative to project root).
  pub folder: String,
  /// All entries (ID → label) declared in the `contains::` block.
  pub entries: Vec<Entry>,
}

/// Namespace loader — discovers and indexes all project symbols.
pub struct NamespaceLoader {
  /// Project root directory.
  root: PathBuf,
  /// Accumulated symbol table across all `name.aura` files.
  pub table: SymbolTable,
  /// All discovered namespace manifests.
  pub manifests: Vec<Manifest>,
}

impl NamespaceLoader {
  /// Creates a new loader anchored at the given project root.
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self {
      root: root.into(),
      table: SymbolTable::new(),
      manifests: Vec::new(),
    }
  }

  /// Reads and indexes all `name.aura` files starting from the
  /// project root and recursing into standard sub-folders.
  pub fn load(&mut self) -> Result<()> {
    // Standard sub-folders that may contain `name.aura`.
    let dirs = [
      "", // project root
      "info",
      "meta",
      "tracks",
      "episodes",
      "scenes",
      "variants",
      "acts",
      "seasons",
      "chapters",
      "segments",
      "interviews",
    ];

    for dir in &dirs {
      let path = if dir.is_empty() {
        self.root.join("name.aura")
      } else {
        self.root.join(dir).join("name.aura")
      };

      if path.exists() {
        self.load_one(&path)?;
      }
    }

    Ok(())
  }

  /// Loads a single `name.aura` and registers its entries.
  pub fn load_one(&mut self, path: &Path) -> Result<()> {
    let text = std::fs::read_to_string(path)
      .map_err(|e| CompileError::msg(format!("cannot read `{}`: {}", path.display(), e)))?;

    let manifest = parse_name_file(&text, path)?;

    for entry in &manifest.entries {
      let key = format!("{}/{}", manifest.folder, entry.id);
      self.table.insert(key, path.to_path_buf());
    }

    self.manifests.push(manifest);
    Ok(())
  }
}

/// Minimal parser for `name.aura` — extracts folder and entry IDs.
///
/// Handles two forms:
///
/// **Root `name.aura`** (project entry) — extracts the project ID
/// from the `name::id ->` field and registers it:
/// ```aura
/// name::
///   id -> c8xab3d
///   slug -> signal-loss
/// ```
///
/// **Sub-folder `name.aura`** (folder index) — extracts `folder` name
/// and `contains::` entries:
/// ```aura
/// name::
///   folder -> tracks
/// contains::
///   t7xab3c -> "Signal Loss"
/// ```
fn parse_name_file(text: &str, path: &Path) -> Result<Manifest> {
  let mut folder = String::new();
  let mut entries = Vec::new();

  let mut in_name_block = false;
  let mut in_contains = false;

  for line in text.lines() {
    let trimmed = line.trim();

    // Skip comments and dividers
    if trimmed.starts_with("##") || trimmed.starts_with("--") || trimmed.is_empty() {
      continue;
    }

    // Block openers
    if trimmed == "name::" {
      in_name_block = true;
      in_contains = false;
      continue;
    }
    if trimmed == "contains::" {
      in_contains = true;
      in_name_block = false;
      continue;
    }
    // Any other block opener resets context
    if trimmed.ends_with("::") {
      in_name_block = false;
      in_contains = false;
      continue;
    }

    // Inside name:: block: look for `folder ->` or `id ->`
    if in_name_block {
      if let Some(rest) = trimmed.strip_prefix("folder") {
        if let Some(val) = extract_arrow_value(rest) {
          folder = val.to_string();
        }
        continue;
      }
      // `id ->` in root name.aura — register as the root project ID
      if let Some(rest) = trimmed.strip_prefix("id") {
        if let Some(val) = extract_arrow_value(rest) {
          // Root name.aura has id at root level — register as root entry
          entries.push(Entry {
            id: val.to_string(),
            label: None,
          });
        }
        continue;
      }
      continue;
    }

    // Inside contains:: block: `id -> "Label"` or `- id`
    if in_contains {
      // `id -> "Label"` form
      if let Some(arrow_pos) = trimmed.find("->") {
        let id = trimmed[..arrow_pos].trim().to_string();
        if !id.is_empty() && !id.starts_with('#') {
          let label_raw = trimmed[arrow_pos + 2..].trim();
          let label = label_raw.trim_matches('"').to_string();
          entries.push(Entry {
            id,
            label: Some(label),
          });
        }
      }
      // `- id` list form
      else if let Some(id) = trimmed.strip_prefix("- ") {
        entries.push(Entry {
          id: id.trim().to_string(),
          label: None,
        });
      }
    }
  }

  if folder.is_empty() {
    // Infer folder from the path: the parent directory name.
    folder = path
      .parent()
      .and_then(|p| p.file_name())
      .and_then(|n| n.to_str())
      .unwrap_or(".")
      .to_string();
  }

  Ok(Manifest { folder, entries })
}

fn extract_arrow_value(s: &str) -> Option<&str> {
  let rest = s.trim_start().strip_prefix("->")?;
  Some(rest.trim().trim_matches('"'))
}
