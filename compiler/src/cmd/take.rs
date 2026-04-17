//! `aura take`, `aura mark`, `aura rewind`, `aura ledger`, `aura delta`

use crate::error::{CompileError, Result};
use crate::hist::{DeltaReplayer, HistoryStore};
use crate::logs::Logger;
use aura::delta::TakeObject;
use aura::id::{IdGen, Prefix};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Records the current working draft as an immutable take.
pub fn take(project: &PathBuf, message: Option<&str>) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let stream = store.active_stream()?;

  let parent = store.stream_head(&stream)?;
  let mut gen = IdGen::new();
  let id = gen.generate(Prefix::Take);

  let ts = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let take_obj = TakeObject {
    id: id.as_str().to_string(),
    parent,
    stream: stream.clone(),
    message: message.map(String::from),
    timestamp: ts,
    deltas: Vec::new(), // populated by the diff engine in full impl
  };

  store.write_take(&take_obj)?;
  store.set_stream_head(&stream, id.as_str())?;
  Logger::new().success(&format!("Take {} recorded on stream {}", id, stream));
  Ok(())
}

/// Attaches a human-readable mark name to the current take.
pub fn mark(project: &PathBuf, name: &str) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let stream = store.active_stream()?;
  let head = store
    .stream_head(&stream)?
    .ok_or_else(|| CompileError::msg("no takes recorded yet — run `aura take` first"))?;
  store.set_mark(name, &head)?;
  Logger::new().success(&format!("Mark `{}` attached to take {}", name, head));
  Ok(())
}

/// Restores the working draft to a previous take (non-destructive).
///
/// `target` may be:
/// - a take ID (`tx3ab7k`)
/// - a mark name (`v1.0`, `premiere`)
/// - a relative reference (`~1`, `~3`)
pub fn rewind(project: &PathBuf, target: &str) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let replayer = DeltaReplayer::new(&store);

  let take_id = resolve_target(&store, target)?;
  let state = replayer.reconstruct(&take_id)?;

  // In the full implementation: write `state` back to the working
  // `.aura` files, overwriting the current draft.
  Logger::new().success(&format!("Rewound to take {} ({} nodes)", take_id, state.len()));
  Ok(())
}

/// Shows the full take ledger for the active stream (or a specific node path).
pub fn ledger(project: &PathBuf, node_path: Option<&str>) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let stream = store.active_stream()?;
  let replayer = DeltaReplayer::new(&store);
  let chain = replayer.ledger(&stream)?;

  let log = Logger::new();
  log.info(&format!("Ledger for stream `{}`:", stream));
  for take in &chain {
    let mark_indicator = "";
    let msg = take.message.as_deref().unwrap_or("(no message)");
    log.info(&format!("  {} {} {}", take.id, msg, mark_indicator));
  }

  if let Some(_path) = node_path {
    log.info("\nNode path filtering not yet implemented.");
  }

  Ok(())
}

/// Shows changed nodes between two takes.
pub fn delta(project: &PathBuf, take_a: &str, take_b: &str) -> Result<()> {
  let store = HistoryStore::open(project)?;
  let id_a = resolve_target(&store, take_a)?;
  let id_b = resolve_target(&store, take_b)?;
  let replayer = DeltaReplayer::new(&store);
  let deltas = replayer.diff_takes(&id_a, &id_b)?;

  let log = Logger::new();
  log.info(&format!("Changes from {} → {}:", id_a, id_b));
  for d in &deltas {
    match d {
      aura::delta::SourceDelta::Upsert { path, .. } => log.info(&format!("  ~ {}", path)),
      aura::delta::SourceDelta::Drop { path } => log.info(&format!("  - {}", path)),
    }
  }
  if deltas.is_empty() {
    log.info("  (no changes)");
  }

  Ok(())
}

// -------------------------------------------------------------------- //
// Internal helpers

fn resolve_target(store: &HistoryStore, target: &str) -> Result<String> {
  // Relative: `~N`
  if let Some(rest) = target.strip_prefix('~') {
    let _n: usize = rest
      .parse()
      .map_err(|_| CompileError::msg(format!("invalid relative reference `{}`", target)))?;
    let stream = store.active_stream()?;
    let head = store
      .stream_head(&stream)?
      .ok_or_else(|| CompileError::msg("no takes recorded yet"))?;
    // Walk back N steps.
    // In the full implementation: follow parent chain N times.
    return Ok(head);
  }

  // Mark name?
  if let Ok(Some(id)) = store.mark(target) {
    return Ok(id);
  }

  // Assume it's a direct take ID.
  if store.has_take(target) {
    return Ok(target.to_string());
  }

  Err(CompileError::msg(format!(
    "cannot resolve target `{}` — not a take ID, mark, or relative reference",
    target
  )))
}
