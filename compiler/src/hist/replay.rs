//! Delta chain replayer — reconstructs virtual source state for any take.
//!
//! Reconstruction works by replaying the chain of `SourceDelta` objects
//! from the origin take to the target take. The result is an in-memory
//! node map representing the full AURA document state at that take —
//! without writing any file to disk.
//!
//! # Algorithm
//!
//! ```text
//! origin (tx3ab7k)
//!   delta: Upsert { verse/one/line/one, "The signal..." }
//!
//! tx3ab8m (parent: tx3ab7k)
//!   delta: Upsert { verse/one/line/two, "She said..." }
//!
//! tx3ab9n (parent: tx3ab8m)
//!   delta: Upsert { chorus/one/line/one, "Find me..." }
//!          Drop   { bridge/one }
//!
//! Reconstruct tx3ab9n:
//!   state = {} (empty)
//!   apply tx3ab7k deltas → { verse/one/line/one: "..." }
//!   apply tx3ab8m deltas → { verse/one/line/one: ..., verse/one/line/two: ... }
//!   apply tx3ab9n deltas → { ..., chorus/one/line/one: ..., bridge/one removed }
//! ```

use crate::error::{CompileError, Result};
use crate::hist::{delta::DeltaEngine, store::HistoryStore};
use aura::delta::TakeObject;
use std::collections::HashMap;

/// Delta chain replayer.
pub struct DeltaReplayer<'a> {
  store: &'a HistoryStore,
}

impl<'a> DeltaReplayer<'a> {
  pub fn new(store: &'a HistoryStore) -> Self {
    Self { store }
  }

  /// Reconstructs the full AURA node map at the given take ID.
  ///
  /// Returns a `HashMap<path, aura_text>` suitable for passing to the
  /// compiler pipeline as a virtual source file.
  pub fn reconstruct(&self, target_id: &str) -> Result<HashMap<String, String>> {
    // Build the chain from origin to target.
    let chain = self.build_chain(target_id)?;

    // Replay chain forward from an empty base.
    let mut state: HashMap<String, String> = HashMap::new();
    for take in &chain {
      state = DeltaEngine::apply(state, &take.deltas);
    }

    Ok(state)
  }

  /// Returns the chain of takes from origin to `target_id` in
  /// chronological order (origin first, target last).
  fn build_chain(&self, target_id: &str) -> Result<Vec<TakeObject>> {
    let mut chain: Vec<TakeObject> = Vec::new();
    let mut current_id = target_id.to_string();

    loop {
      let take = self.store.read_take(&current_id)?;
      let parent = take.parent.clone();
      chain.push(take);
      match parent {
        Some(p) => current_id = p,
        None => break, // reached origin
      }
    }

    // Reverse so origin is first.
    chain.reverse();
    Ok(chain)
  }

  /// Returns a human-readable ledger of all takes from origin to the
  /// head of the given stream.
  pub fn ledger(&self, stream: &str) -> Result<Vec<TakeObject>> {
    let head = self
      .store
      .stream_head(stream)?
      .ok_or_else(|| CompileError::msg(format!("stream `{}` not found", stream)))?;
    self.build_chain(&head)
  }

  /// Returns the diff between two specific takes.
  pub fn diff_takes(&self, take_a: &str, take_b: &str) -> Result<Vec<aura::delta::SourceDelta>> {
    let state_a = self.reconstruct(take_a)?;
    let state_b = self.reconstruct(take_b)?;
    Ok(DeltaEngine::diff(&state_a, &state_b))
  }
}
