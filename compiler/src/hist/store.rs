//! History object store reader/writer.
//!
//! Manages the `.history/` append-only object store at the project root.
//!
//! # On-disk layout
//!
//! ```text
//! .history/
//!   objects/
//!     tx3ab7k.toml     <- immutable take objects in TOML format
//!     tx9zz1p.toml
//!   marks/
//!     v1.0             <- plain text: take ID
//!     premiere         <- plain text: take ID
//!   streams/
//!     main             <- plain text: head take ID for the stream
//!     translation-fr
//!   HEAD               <- plain text: active stream name
//! ```
//!
//! `configs/` is never tracked. Binary assets are never tracked.
//! Only `.aura` source files and their AST-level diffs are stored.

use crate::error::{CompileError, Result};
use crate::hist::serial::TakeToml;
use aura::delta::{MarkEntry, TakeObject};
use std::fs;
use std::path::{Path, PathBuf};

/// The `.history/` object store.
pub struct HistoryStore {
  root: PathBuf,
}

impl HistoryStore {
  /// Opens or creates the `.history/` store at the given project root.
  pub fn open(project_root: impl Into<PathBuf>) -> Result<Self> {
    let root = project_root.into().join(".history");
    if !root.exists() {
      fs::create_dir_all(root.join("objects"))
        .and_then(|_| fs::create_dir_all(root.join("marks")))
        .and_then(|_| fs::create_dir_all(root.join("streams")))
        .map_err(|e| CompileError::msg(format!("cannot create .history/: {}", e)))?;
      fs::write(root.join("HEAD"), "main")
        .map_err(|e| CompileError::msg(format!("cannot write HEAD: {}", e)))?;
    }
    Ok(Self { root })
  }

  // ---------------------------------------------------------------- //
  // HEAD / stream management

  pub fn active_stream(&self) -> Result<String> {
    self.read_text(self.root.join("HEAD"))
  }

  pub fn set_stream(&self, stream: &str) -> Result<()> {
    self.write_text(self.root.join("HEAD"), stream)
  }

  pub fn stream_head(&self, stream: &str) -> Result<Option<String>> {
    let path = self.root.join("streams").join(stream);
    if path.exists() {
      Ok(Some(self.read_text(path)?))
    } else {
      Ok(None)
    }
  }

  pub fn set_stream_head(&self, stream: &str, take_id: &str) -> Result<()> {
    self.write_text(self.root.join("streams").join(stream), take_id)
  }

  pub fn list_streams(&self) -> Result<Vec<String>> {
    self.list_dir_names(self.root.join("streams"))
  }

  // ---------------------------------------------------------------- //
  // Marks

  pub fn set_mark(&self, name: &str, take_id: &str) -> Result<()> {
    self.write_text(self.root.join("marks").join(name), take_id)
  }

  pub fn mark(&self, name: &str) -> Result<Option<String>> {
    let path = self.root.join("marks").join(name);
    if path.exists() {
      Ok(Some(self.read_text(path)?))
    } else {
      Ok(None)
    }
  }

  pub fn list_marks(&self) -> Result<Vec<MarkEntry>> {
    let names = self.list_dir_names(self.root.join("marks"))?;
    let mut marks = Vec::new();
    for name in names {
      let take = self.read_text(self.root.join("marks").join(&name))?;
      marks.push(MarkEntry { name, take });
    }
    Ok(marks)
  }

  // ---------------------------------------------------------------- //
  // Take objects  (TOML serialization via hist::serial)

  /// Writes a `TakeObject` to `.history/objects/{id}.toml`.
  pub fn write_take(&self, take: &TakeObject) -> Result<()> {
    let mirror: TakeToml = take.clone().into();
    let toml_str = toml::to_string_pretty(&mirror)
      .map_err(|e| CompileError::msg(format!("cannot serialize take {}: {}", take.id, e)))?;
    let path = self.root.join("objects").join(format!("{}.toml", take.id));
    self.write_text(path, &toml_str)
  }

  /// Reads a `TakeObject` from `.history/objects/{id}.toml`.
  pub fn read_take(&self, id: &str) -> Result<TakeObject> {
    let path = self.root.join("objects").join(format!("{}.toml", id));
    if !path.exists() {
      return Err(CompileError::msg(format!(
        "take `{}` not found in .history/objects/",
        id
      )));
    }
    let text = self.read_text(path)?;
    let mirror: TakeToml = toml::from_str(&text)
      .map_err(|e| CompileError::msg(format!("cannot parse take {}: {}", id, e)))?;
    Ok(mirror.into())
  }

  pub fn has_take(&self, id: &str) -> bool {
    self
      .root
      .join("objects")
      .join(format!("{}.toml", id))
      .exists()
  }

  pub fn list_takes(&self) -> Result<Vec<String>> {
    self.list_dir_names(self.root.join("objects")).map(|names| {
      names
        .into_iter()
        .filter_map(|n| n.strip_suffix(".toml").map(String::from))
        .collect()
    })
  }

  // ---------------------------------------------------------------- //
  // Helpers

  fn read_text(&self, path: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(path.as_ref())
      .map_err(|e| CompileError::msg(format!("cannot read `{}`: {}", path.as_ref().display(), e)))
      .map(|s| s.trim().to_string())
  }

  fn write_text(&self, path: impl AsRef<Path>, text: &str) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
      fs::create_dir_all(parent)
        .map_err(|e| CompileError::msg(format!("cannot create directory: {}", e)))?;
    }
    fs::write(path.as_ref(), text)
      .map_err(|e| CompileError::msg(format!("cannot write `{}`: {}", path.as_ref().display(), e)))
  }

  fn list_dir_names(&self, dir: impl AsRef<Path>) -> Result<Vec<String>> {
    let dir = dir.as_ref();
    if !dir.exists() {
      return Ok(Vec::new());
    }
    let entries = fs::read_dir(dir).map_err(|e| {
      CompileError::msg(format!("cannot read directory `{}`: {}", dir.display(), e))
    })?;
    let mut names = Vec::new();
    for entry in entries.flatten() {
      if let Some(name) = entry.file_name().to_str() {
        names.push(name.to_string());
      }
    }
    names.sort();
    Ok(names)
  }
}
