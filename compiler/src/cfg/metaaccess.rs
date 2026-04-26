//! ReBAC access weight loader — reads `meta/metaaccess.aura`.
//!
//! Parses a Directed Acyclic Graph (DAG) of custom access tiers and
//! executes Kahn's topological sort to assign deterministic integer
//! weights. Weights are packed into the upper 16 bits of each
//! `AtomNode.node_class` at emit time, enabling branchless SIMD access
//! checks in the engine:
//!
//! ```text
//!   caller_weight >= node.access_weight   →  access granted
//! ```
//!
//! # File format
//!
//! ```aura
//! ## FILE: meta/metaaccess.aura
//!
//! access-dag::
//!
//!   open::
//!     weight -> 1
//!
//!   archived::
//!     weight  -> 2
//!     extends -> open
//!
//!   restricted::
//!     weight  -> 3
//!     extends -> archived
//!
//!   press-only::
//!     weight  -> 3
//!     extends -> archived
//!
//!   gated::
//!     weight  -> 4
//!     extends -> restricted
//!
//!   premium-only::
//!     weight  -> 4
//!     extends -> gated
//!
//!   embargoed::
//!     weight  -> 5
//!     extends -> gated
//!
//!   locked::
//!     weight  -> 6
//!     extends -> embargoed
//! ```
//!
//! Custom tiers (e.g. `press-only`, `premium-only`) become first-class
//! access levels without any compiler or engine change.
//!
//! Falls back to `AccessWeights::builtin()` (the six fixed tiers) if
//! `meta/metaaccess.aura` is absent or malformed.

use std::collections::{HashMap, VecDeque};
use std::path::Path;

// -------------------------------------------------------------------- //
// Public API

/// Access weight table produced by the topological sort.
///
/// Maps tier name → u16 integer weight. Weights are monotonically
/// increasing from root (lowest restriction) to leaf (highest restriction).
#[derive(Debug, Clone)]
pub struct AccessWeights {
  inner: HashMap<String, u16>,
}

impl AccessWeights {
  /// Returns the built-in six-tier fallback weights (no `metaaccess.aura`).
  pub fn builtin() -> Self {
    let mut m = HashMap::new();
    m.insert("open".into(), 1);
    m.insert("archived".into(), 2);
    m.insert("restricted".into(), 3);
    m.insert("gated".into(), 4);
    m.insert("embargoed".into(), 5);
    m.insert("locked".into(), 6);
    Self { inner: m }
  }

  /// Returns the weight for `tier`, or `None` if unrecognized.
  pub fn get(&self, tier: &str) -> Option<u16> {
    self.inner.get(tier).copied()
  }

  /// Returns the weight for `tier`, falling back to `0` (no restriction).
  ///
  /// A weight of `0` in the ATOM struct means "caller weight is always ≥
  /// node weight" — effectively public access.
  pub fn resolve(&self, tier: &str) -> u16 {
    self.inner.get(tier).copied().unwrap_or(0)
  }

  /// Packs class byte and access weight into a single `u32` for storage
  /// in `AtomNode.node_class`.
  ///
  /// Layout:
  /// ```text
  /// bits 31-16: access_weight (u16)
  /// bits 15-0:  class_byte    (u16, upper byte is 0)
  /// ```
  pub fn pack(class_byte: u32, access_weight: u16) -> u32 {
    ((access_weight as u32) << 16) | (class_byte & 0xFFFF)
  }

  /// Unpacks the class byte from a packed `node_class` field.
  pub fn unpack_class(packed: u32) -> u32 {
    packed & 0xFFFF
  }

  /// Unpacks the access weight from a packed `node_class` field.
  pub fn unpack_weight(packed: u32) -> u16 {
    ((packed >> 16) & 0xFFFF) as u16
  }
}

/// Loads `meta/metaaccess.aura` from the project root.
///
/// Falls back to [`AccessWeights::builtin`] on any error.
pub fn load(project: &Path) -> AccessWeights {
  let path = project.join("meta").join("metaaccess.aura");
  if !path.exists() {
    return AccessWeights::builtin();
  }
  match std::fs::read_to_string(&path) {
    Ok(text) => parse_and_sort(&text).unwrap_or_else(|_| AccessWeights::builtin()),
    Err(_) => AccessWeights::builtin(),
  }
}

// -------------------------------------------------------------------- //
// Internal DAG representation

/// Intermediate representation of one tier in the DAG.
#[derive(Debug, Default)]
struct DagNode {
  /// Explicitly declared weight (optional — computed if absent).
  explicit_weight: Option<u16>,
  /// Names of tiers this one extends (parent edges).
  extends: Vec<String>,
}

// -------------------------------------------------------------------- //
// Kahn's topological sort

/// Parses the `access-dag::` block and runs Kahn's algorithm.
fn parse_and_sort(text: &str) -> Result<AccessWeights, &'static str> {
  let mut nodes: HashMap<String, DagNode> = HashMap::new();
  let mut current: Option<String> = None;
  let mut in_dag = false;

  for line in text.lines() {
    let trimmed = line.trim();

    if trimmed.is_empty() || trimmed.starts_with("##") || trimmed.starts_with("--") {
      continue;
    }

    // Top-level `access-dag::` block.
    if trimmed == "access-dag::" {
      in_dag = true;
      current = None;
      continue;
    }

    if !in_dag {
      continue;
    }

    // Tier opener: `open::`, `press-only::`, etc.
    if let Some(name) = trimmed.strip_suffix("::") {
      let name = name.trim().to_string();
      nodes.entry(name.clone()).or_default();
      current = Some(name);
      continue;
    }

    // Field inside a tier: `weight -> 3` or `extends -> open`.
    if let Some(arrow) = trimmed.find("->") {
      let key = trimmed[..arrow].trim();
      let val = trimmed[arrow + 2..].trim().trim_matches('"');
      if let Some(ref tier) = current.clone() {
        let node = nodes.entry(tier.clone()).or_default();
        match key {
          "weight" => {
            if let Ok(w) = val.parse::<u16>() {
              node.explicit_weight = Some(w);
            }
          }
          "extends" => {
            node.extends.push(val.to_string());
          }
          _ => {}
        }
      }
    }
  }

  if nodes.is_empty() {
    return Err("access-dag is empty");
  }

  // ---------------------------------------------------------------- //
  // Kahn's algorithm

  let names: Vec<String> = nodes.keys().cloned().collect();

  // Build: parent → [children], in-degree per child.
  let mut children: HashMap<String, Vec<String>> = HashMap::new();
  let mut in_degree: HashMap<String, usize> = HashMap::new();

  for n in &names {
    in_degree.entry(n.clone()).or_insert(0);
  }

  for (name, node) in &nodes {
    for parent in &node.extends {
      children
        .entry(parent.clone())
        .or_default()
        .push(name.clone());
      *in_degree.entry(name.clone()).or_insert(0) += 1;
    }
  }

  // Roots: nodes with zero in-degree (no parents).
  let mut queue: VecDeque<String> = in_degree
    .iter()
    .filter(|(_, &d)| d == 0)
    .map(|(n, _)| n.clone())
    .collect();

  // Sort roots for deterministic output.
  let mut roots: Vec<String> = queue.drain(..).collect();
  roots.sort();
  queue.extend(roots);

  let mut order: Vec<String> = Vec::with_capacity(names.len());

  while let Some(n) = queue.pop_front() {
    order.push(n.clone());
    if let Some(ch) = children.get(&n) {
      let mut sorted_ch = ch.clone();
      sorted_ch.sort();
      for child in sorted_ch {
        let deg = in_degree.entry(child.clone()).or_insert(0);
        *deg = deg.saturating_sub(1);
        if *deg == 0 {
          queue.push_back(child);
        }
      }
    }
  }

  if order.len() != names.len() {
    return Err("access-dag contains a cycle — cannot topologically sort");
  }

  // ---------------------------------------------------------------- //
  // Weight assignment: explicit weights win; others = max(parent) + 1.

  let mut weights: HashMap<String, u16> = HashMap::new();

  for name in &order {
    let node = &nodes[name];
    let w = if let Some(w) = node.explicit_weight {
      w
    } else {
      let parent_max = node
        .extends
        .iter()
        .filter_map(|p| weights.get(p).copied())
        .max()
        .unwrap_or(0);
      parent_max + 1
    };
    weights.insert(name.clone(), w);
  }

  Ok(AccessWeights { inner: weights })
}
