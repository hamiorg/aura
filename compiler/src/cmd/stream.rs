//! `aura stream` and `aura mix` — parallel stream management.

use crate::error::{CompileError, Result};
use crate::hist::HistoryStore;
use crate::logs::Logger;
use std::path::PathBuf;

/// Opens a new named development stream branching from the current take.
pub fn open(project: &PathBuf, name: &str) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let stream = store.active_stream()?;
  let head = store.stream_head(&stream)?.ok_or_else(|| {
    CompileError::msg("no takes recorded yet — record a take before opening a stream")
  })?;

  // New stream starts at the current head.
  store.set_stream_head(name, &head)?;
  store.set_stream(name)?;

  Logger::new().success(&format!("Opened stream `{}` at take {}", name, head));
  Ok(())
}

/// Closes and archives a named stream.
pub fn close(project: &PathBuf, name: &str) -> Result<()> {
  if name == "main" {
    return Err(CompileError::msg("cannot close the main stream"));
  }
  let store = HistoryStore::open(project)?;
  let active = store.active_stream()?;
  if active == name {
    store.set_stream("main")?;
    Logger::new().info("Switched active stream back to main");
  }
  // In the full implementation: move stream pointer file to an
  // `archived/` folder. Scaffold: just print.
  Logger::new().info(&format!("Stream `{}` closed", name));
  Ok(())
}

/// Lists all open streams.
pub fn list(project: &PathBuf) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let active = store.active_stream()?;
  let streams = store.list_streams()?;

  let log = Logger::new();
  log.info("Streams:");
  for s in &streams {
    let marker = if s == &active { " *" } else { "" };
    log.info(&format!("  {}{}", s, marker));
  }
  Ok(())
}

/// Merges a completed stream into the current active stream.
///
/// In the full implementation, this applies the stream's delta chain
/// to the current draft, resolving any conflicts interactively.
pub fn mix(project: &PathBuf, stream_name: &str) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let active = store.active_stream()?;

  if active == stream_name {
    return Err(CompileError::msg("cannot mix a stream into itself"));
  }

  // The mix algorithm:
  // 1. Reconstruct the current head of `stream_name`
  // 2. Reconstruct the current head of `active`
  // 3. Diff the two states from their common ancestor
  // 4. Apply non-conflicting deltas to the working draft
  // 5. Report conflicts for manual resolution
  // 5. Report conflicts for manual resolution
  let log = Logger::new();
  log.info(&format!("Mixing stream `{}` into `{}`", stream_name, active));
  log.note("(mix algorithm not yet implemented — scaffold only)");
  Ok(())
}
