//! ATOM flat-array interval tree emitter — produces `.atom` files.
//!
//! # `.atom` file layout
//!
//! ```text
//! ┌────────────────────────────────────────────────────────┐
//! │  Magic: "ATOM" (4 bytes)                               │
//! │  Version: u16                                          │
//! │  Node count: u32                                       │
//! ├────────────────────────────────────────────────────────┤
//! │  AtomNode[0]  { low, high, duration, max, ptr, class } │
//! │  AtomNode[1]  { ... }                                  │
//! │  AtomNode[N]  { ... }                                  │
//! ├────────────────────────────────────────────────────────┤
//! │  String Pool                                           │
//! │  (null-terminated UTF-8 strings; data_ptr indexes here)│
//! └────────────────────────────────────────────────────────┘
//! ```
//!
//! # Two-pass algorithm
//!
//! **Pass 1 — build:** Walk the resolved AST and flatten all interval-
//! indexed nodes into a `Vec<AtomNode>` ordered by `low`.
//!
//! **Pass 2 — max:** Fill the `max` field bottom-up (augmented interval
//! tree property). For leaf `i`, `max[i] = high[i]`. For internal node
//! `i`, `max[i] = max(high[i], max[left(i)], max[right(i)])`.
//!
//! # SIMD alignment
//!
//! Each `AtomNode` is 24 bytes. One AVX-2 register (256-bit) holds 10.67
//! nodes — the SIMD loop processes 8-node blocks, covering `low`, `high`,
//! and `duration` of two adjacent nodes per cycle.

use crate::cfg::AccessWeights;
use crate::error::{CompileError, Result};
use crate::parse::ast::{Child, Document, Namespace, NodeType, Value};
use aura::interval::Interval;
use aura::node::{class, AtomNode};

// -------------------------------------------------------------------- //
// Wire constants

pub const MAGIC: &[u8; 4] = b"ATOM";
pub const VERSION: u16 = 1;

// Size of the fixed header: 4 (magic) + 2 (version) + 4 (count) = 10 bytes.
pub const HEADER_SIZE: usize = 10;

// -------------------------------------------------------------------- //
// String pool

/// Append-only string pool for the string section of the `.atom` file.
struct Pool {
  data: Vec<u8>,
  offsets: std::collections::HashMap<String, u32>,
}

impl Pool {
  fn new() -> Self {
    Self {
      data: Vec::new(),
      offsets: std::collections::HashMap::new(),
    }
  }

  /// Interns a string and returns its byte offset in the pool.
  fn intern(&mut self, s: &str) -> u32 {
    if let Some(&off) = self.offsets.get(s) {
      return off;
    }
    let off = self.data.len() as u32;
    self.data.extend_from_slice(s.as_bytes());
    self.data.push(0); // null terminator
    self.offsets.insert(s.to_string(), off);
    off
  }
}

// -------------------------------------------------------------------- //
// Emitter

/// ATOM interval tree emitter.
pub struct AtomEmitter {
  /// Flat array of nodes built in Pass 1 (ordered by `low`).
  nodes: Vec<AtomNode>,
  /// String pool accumulating text data.
  pool: Pool,
  /// ReBAC access weight table — packed into upper 16 bits of `node_class`.
  /// `None` uses only the built-in six-tier fallback.
  access: AccessWeights,
}

impl AtomEmitter {
  pub fn new() -> Self {
    Self {
      nodes: Vec::with_capacity(256),
      pool: Pool::new(),
      access: AccessWeights::builtin(),
    }
  }

  /// Constructs an emitter pre-loaded with project-specific access weights.
  ///
  /// Weights are derived from `meta/metaaccess.aura` (or the built-in
  /// six-tier fallback) via [`crate::cfg::metaaccess::load`].
  pub fn with_access_weights(weights: AccessWeights) -> Self {
    Self {
      nodes: Vec::with_capacity(256),
      pool: Pool::new(),
      access: weights,
    }
  }

  /// Emits the document and returns the complete `.atom` bytes.
  pub fn emit(&mut self, doc: &Document<'_>) -> Result<Vec<u8>> {
    // Pass 1 — build flat node array from AST.
    for ns in &doc.namespaces {
      self.visit_namespace(ns)?;
    }

    // Sort nodes by `low` value (required for BST layout).
    self.nodes.sort_by(|a, b| {
      a.low
        .partial_cmp(&b.low)
        .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Pass 2 — fill `max` values bottom-up.
    self.compute_max();

    // Serialize.
    let count = self.nodes.len() as u32;
    let mut out = Vec::with_capacity(
      HEADER_SIZE + self.nodes.len() * std::mem::size_of::<AtomNode>() + self.pool.data.len(),
    );

    // Header
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());

    // Node array
    for node in &self.nodes {
      out.extend_from_slice(&node.low.to_le_bytes());
      out.extend_from_slice(&node.high.to_le_bytes());
      out.extend_from_slice(&node.duration.to_le_bytes());
      out.extend_from_slice(&node.max.to_le_bytes());
      out.extend_from_slice(&node.data_ptr.to_le_bytes());
      out.extend_from_slice(&node.node_class.to_le_bytes());
    }

    // String pool
    out.extend_from_slice(&self.pool.data);

    Ok(out)
  }

  fn visit_namespace(&mut self, ns: &Namespace<'_>) -> Result<()> {
    if !ns.node_type.is_interval() {
      // Non-interval namespaces (manifest, schema, etc.) are
      // handled by the HAMI emitter, not the ATOM emitter.
      // Still recurse into children to find interval nodes.
      for child in &ns.children {
        if let Child::Block(b) = child {
          self.visit_namespace(b)?;
        }
      }
      return Ok(());
    }

    // Extract the `time` field from this namespace's children.
    let interval = extract_interval(ns)?;
    let data_ptr = self.pool.intern(&ns.path);
    let class_byte = node_type_to_class(ns.node_type);

    // Resolve access weight from any `access -> @access/<tier>` field.
    // Pack weight into the upper 16 bits of node_class:
    //   bits 31-16: access_weight (u16, 0 = unrestricted)
    //   bits 15-0:  class_byte
    let access_weight = extract_access_weight(ns, &self.access);
    let node_class = AccessWeights::pack(class_byte, access_weight);

    let node = AtomNode::new(interval.low, interval.high, data_ptr, node_class);
    self.nodes.push(node);

    // Recurse into child blocks.
    for child in &ns.children {
      if let Child::Block(b) = child {
        self.visit_namespace(b)?;
      }
    }

    Ok(())
  }

  /// Pass 2 — fills `max` values for the augmented interval tree.
  ///
  /// For a flat-array BST the children of node at index `i` are at
  /// `2*i+1` (left) and `2*i+2` (right). We iterate right-to-left
  /// so children are processed before their parent.
  fn compute_max(&mut self) {
    let n = self.nodes.len();
    for i in (0..n).rev() {
      let mut m = self.nodes[i].high;
      let left = 2 * i + 1;
      let right = 2 * i + 2;
      if left < n {
        m = m.max(self.nodes[left].max);
      }
      if right < n {
        m = m.max(self.nodes[right].max);
      }
      self.nodes[i].max = m;
    }
  }

  /// Returns the number of interval nodes accumulated so far.
  /// Used by compile_one() to decide whether to write the .atom file.
  pub fn node_count(&self) -> usize {
    self.nodes.len()
  }
}

impl Default for AtomEmitter {
  fn default() -> Self {
    Self::new()
  }
}

// -------------------------------------------------------------------- //
// Helpers

/// Scans a namespace's children for `access -> @access/<tier>` and resolves
/// the tier name to a u16 weight using the project's `AccessWeights` table.
///
/// Returns `0` if no access field is present (unrestricted by default).
fn extract_access_weight(ns: &Namespace<'_>, weights: &AccessWeights) -> u16 {
  use crate::parse::ast::{RefBody, Value};
  for child in &ns.children {
    if let Child::Field(f) = child {
      if f.key == "access" {
        if let Value::Ref(r) = &f.value {
          if r.domain == "access" {
            if let RefBody::Single(tier) = &r.body {
              return weights.resolve(tier);
            }
          }
        }
      }
    }
  }
  0
}

/// Extracts the `time` field from a namespace's children and normalizes it.
fn extract_interval(ns: &Namespace<'_>) -> Result<Interval> {
  use crate::parse::time::TimeNorm;

  for child in &ns.children {
    if let Child::Field(f) = child {
      if f.key == "time" {
        if let Value::Time(expr) = &f.value {
          return TimeNorm::normalize(expr);
        }
      }
    }
  }
  // Interval-indexed nodes must have a `time` field.
  Err(CompileError::msg(format!(
    "node `{}` is missing a required `time` field",
    ns.path
  )))
}

/// Maps an AST `NodeType` to an ATOM node class constant.
fn node_type_to_class(nt: NodeType) -> u32 {
  match nt {
    // Content nodes
    NodeType::Act
    | NodeType::Scene
    | NodeType::Shot
    | NodeType::Verse
    | NodeType::Chorus
    | NodeType::Bridge
    | NodeType::Intro
    | NodeType::Outro
    | NodeType::Hook
    | NodeType::Drop
    | NodeType::Interlude
    | NodeType::Breakdown
    | NodeType::PreChorus
    | NodeType::PostChorus
    | NodeType::Chapter
    | NodeType::Segment
    | NodeType::Section
    | NodeType::Line
    | NodeType::Dialogue
    | NodeType::Word
    | NodeType::Token
    | NodeType::Syllable
    | NodeType::Phoneme
    | NodeType::Letter
    | NodeType::Character => class::CONTENT,

    // Support nodes
    NodeType::Segments => class::SEGMENT,
    NodeType::Instruments => class::INSTRUMENT,
    NodeType::Chapters => class::CHAPTER,
    NodeType::Windows => class::CREDIT,
    NodeType::Translations => class::TRANSLATION,
    NodeType::Moods => class::MOOD,
    NodeType::Rights => class::RIGHTS,
    NodeType::Slots => class::SLOT,
    NodeType::Anchors => class::ANCHOR,
    NodeType::Tempo => class::TEMPO,
    NodeType::Samples => class::SAMPLE,
    NodeType::Explainers => class::EXPLAINER,
    NodeType::Interpolations => class::INTERPOLATION,
    NodeType::Instructions => class::INSTRUCTION,
    NodeType::Events => class::EVENT,

    _ => class::CONTENT,
  }
}
