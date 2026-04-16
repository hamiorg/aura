//! History object store reader/writer.
//!
//! Manages the `.history/` append-only object store at the project root.
//!
//! # On-disk layout
//!
//! ```text
//! .history/
//!   objects/         <- immutable take objects serialized as .hami
//!     tx3ab7k.hami
//!     tx9zz1p.hami
//!   marks/
//!     v1.0           <- plain text: take ID "tx3ab8m"
//!     premiere       <- plain text: take ID "tx3ab7k"
//!   streams/
//!     main           <- plain text: head take ID for the main stream
//!     translation-fr <- plain text: head take ID for a parallel stream
//!   HEAD             <- plain text: active stream name
//! ```
//!
//! `configs/` is **never** tracked. Binary assets are **never** tracked.
//! Only `.aura` source files and their AST node diffs are stored.

use crate::error::{CompileError, Result};
use aura::delta::{MarkEntry, TakeObject};
use std::fs;
use std::path::{Path, PathBuf};

/// The `.history/` object store.
pub struct HistoryStore {
  /// Path to the `.history/` directory.
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
      // Initialize HEAD pointing to the main stream.
      fs::write(root.join("HEAD"), "main")
        .map_err(|e| CompileError::msg(format!("cannot write HEAD: {}", e)))?;
    }
    Ok(Self { root })
  }

  // ---------------------------------------------------------------- //
  // HEAD / stream management

  /// Returns the name of the currently active stream.
  pub fn active_stream(&self) -> Result<String> {
    self.read_text(self.root.join("HEAD"))
  }

  /// Sets the active stream.
  pub fn set_stream(&self, stream: &str) -> Result<()> {
    self.write_text(self.root.join("HEAD"), stream)
  }

  /// Returns the head take ID for the given stream, if the stream exists.
  pub fn stream_head(&self, stream: &str) -> Result<Option<String>> {
    let path = self.root.join("streams").join(stream);
    if path.exists() {
      Ok(Some(self.read_text(path)?))
    } else {
      Ok(None)
    }
  }

  /// Sets the head take ID for the given stream.
  pub fn set_stream_head(&self, stream: &str, take_id: &str) -> Result<()> {
    let path = self.root.join("streams").join(stream);
    self.write_text(path, take_id)
  }

  /// Returns all open streams by name.
  pub fn list_streams(&self) -> Result<Vec<String>> {
    self.list_dir_names(self.root.join("streams"))
  }

  // ---------------------------------------------------------------- //
  // Marks

  /// Attaches a human-readable mark name to a specific take.
  pub fn set_mark(&self, name: &str, take_id: &str) -> Result<()> {
    let path = self.root.join("marks").join(name);
    self.write_text(path, take_id)
  }

  /// Returns the take ID for a mark, if it exists.
  pub fn mark(&self, name: &str) -> Result<Option<String>> {
    let path = self.root.join("marks").join(name);
    if path.exists() {
      Ok(Some(self.read_text(path)?))
    } else {
      Ok(None)
    }
  }

  /// Returns all mark entries.
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
  // Take objects

  /// Writes a `TakeObject` to `.history/objects/{id}.json`.
  ///
  /// Uses a minimal hand-rolled JSON serialization so `core` stays
  /// dependency-free. In the full implementation this would use a
  /// proper serializer.
  pub fn write_take(&self, take: &TakeObject) -> Result<()> {
    let path = self.root.join("objects").join(format!("{}.json", take.id));
    let json = serialize_take(take);
    self.write_text(path, &json)
  }

  /// Reads a `TakeObject` by ID.
  pub fn read_take(&self, id: &str) -> Result<TakeObject> {
    let path = self.root.join("objects").join(format!("{}.json", id));
    if !path.exists() {
      return Err(CompileError::msg(format!(
        "take `{}` not found in .history/",
        id
      )));
    }
    let text = self.read_text(path)?;
    deserialize_take(&text, id)
  }

  /// Returns `true` if a take with this ID exists in the store.
  pub fn has_take(&self, id: &str) -> bool {
    self
      .root
      .join("objects")
      .join(format!("{}.json", id))
      .exists()
  }

  /// Lists all take IDs in the object store.
  pub fn list_takes(&self) -> Result<Vec<String>> {
    let dir = self.root.join("objects");
    self.list_dir_names(dir).map(|names| {
      names
        .into_iter()
        .filter_map(|n| n.strip_suffix(".json").map(String::from))
        .collect()
    })
  }

  // ---------------------------------------------------------------- //
  // Internal helpers

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

// -------------------------------------------------------------------- //
// Minimal take serialization (no external deps)

fn serialize_take(t: &TakeObject) -> String {
  use aura::delta::SourceDelta;
  let mut s = String::new();
  s.push_str(&format!(
    "{{\"id\":\"{}\",\"stream\":\"{}\",\"timestamp\":{},",
    t.id, t.stream, t.timestamp
  ));
  if let Some(p) = &t.parent {
    s.push_str(&format!("\"parent\":\"{}\",", p));
  }
  if let Some(m) = &t.message {
    s.push_str(&format!("\"message\":\"{}\",", escape_json(m)));
  }
  s.push_str("\"deltas\":[");
  for (i, d) in t.deltas.iter().enumerate() {
    if i > 0 {
      s.push(',');
    }
    match d {
      SourceDelta::Upsert { path, aura } => {
        s.push_str(&format!(
          "{{\"op\":\"upsert\",\"path\":\"{}\",\"aura\":{}}}",
          escape_json(path),
          json_str(aura)
        ));
      }
      SourceDelta::Drop { path } => {
        s.push_str(&format!(
          "{{\"op\":\"drop\",\"path\":\"{}\"}}",
          escape_json(path)
        ));
      }
    }
  }
  s.push_str("]}");
  s
}

fn deserialize_take(_json: &str, id: &str) -> Result<TakeObject> {
  // Minimal parser — in a full implementation use serde_json.
  // Returns a skeleton take so the scaffold compiles cleanly.
  Ok(TakeObject {
    id: id.to_string(),
    parent: None,
    stream: "main".to_string(),
    message: None,
    timestamp: 0,
    deltas: Vec::new(),
  })
}

fn escape_json(s: &str) -> String {
  s.replace('\\', "\\\\")
    .replace('"', "\\\"")
    .replace('\n', "\\n")
}

fn json_str(s: &str) -> String {
  format!("\"{}\"", escape_json(s))
}
