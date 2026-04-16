//! SourceDelta diff engine — computes AST node-level diffs.
//!
//! The delta engine compares two AURA AST documents and produces a
//! `Vec<SourceDelta>` capturing only the changed nodes. Because AURA
//! uses slash-identifiers (e.g. `verse/one/line/two`), the diff is
//! at the AST node level — not raw text line diffs.
//!
//! # Why node-level diffs?
//!
//! - Whitespace-only edits produce no delta.
//! - Reordering fields inside a node produces no delta.
//! - Moving a node to a different position in the file produces no delta
//!   as long as its slash-path is unchanged.
//! - Only semantic changes (new nodes, removed nodes, changed field values)
//!   produce deltas.
//!
//! This keeps delta chains compact and makes history immune to cosmetic
//! reformatting.

use aura::delta::SourceDelta;
use std::collections::HashMap;

/// Computes the delta between `base` (parent take) and `head` (new take).
///
/// `base_nodes` and `head_nodes` are maps from slash-identifier path to
/// the canonical AURA text block for that node.
///
/// Returns a `Vec<SourceDelta>` sufficient to reproduce `head` from `base`
/// by replay.
pub struct DeltaEngine;

impl DeltaEngine {
  /// Computes the delta between two node maps.
  ///
  /// - Nodes in `head` but not `base` → `Upsert` (new)
  /// - Nodes in both with different text → `Upsert` (changed)
  /// - Nodes in `base` but not `head` → `Drop` (removed)
  /// - Nodes identical in both → no delta
  pub fn diff(base: &HashMap<String, String>, head: &HashMap<String, String>) -> Vec<SourceDelta> {
    let mut deltas = Vec::new();

    // New or changed nodes.
    for (path, aura) in head {
      match base.get(path) {
        None => {
          deltas.push(SourceDelta::Upsert {
            path: path.clone(),
            aura: aura.clone(),
          });
        }
        Some(base_aura) if base_aura != aura => {
          deltas.push(SourceDelta::Upsert {
            path: path.clone(),
            aura: aura.clone(),
          });
        }
        _ => {} // unchanged
      }
    }

    // Removed nodes.
    for path in base.keys() {
      if !head.contains_key(path) {
        deltas.push(SourceDelta::Drop { path: path.clone() });
      }
    }

    // Sort for deterministic output (upserts before drops, then by path).
    deltas.sort_by(|a, b| {
      let (ak, ap) = delta_key(a);
      let (bk, bp) = delta_key(b);
      ak.cmp(&bk).then(ap.cmp(bp))
    });

    deltas
  }

  /// Applies a list of deltas to a base node map, producing the head state.
  pub fn apply(
    mut base: HashMap<String, String>,
    deltas: &[SourceDelta],
  ) -> HashMap<String, String> {
    for delta in deltas {
      match delta {
        SourceDelta::Upsert { path, aura } => {
          base.insert(path.clone(), aura.clone());
        }
        SourceDelta::Drop { path } => {
          base.remove(path);
        }
      }
    }
    base
  }
}

fn delta_key(d: &SourceDelta) -> (u8, &str) {
  match d {
    SourceDelta::Upsert { path, .. } => (0, path),
    SourceDelta::Drop { path } => (1, path),
  }
}
