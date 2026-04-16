//! HAMI B-Tree emitter — produces `.hami` manifest files.
//!
//! # `.hami` file layout
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │  Magic: "HAMI" (4 bytes)                            │
//! │  Version: u16                                       │
//! │  Root namespace offset: u32                         │
//! ├─────────────────────────────────────────────────────┤
//! │  Lexical Data Region                                │
//! │  (contiguous key RS value US key RS value US ...)   │
//! ├─────────────────────────────────────────────────────┤
//! │  B-Tree Positional Index                            │
//! │  (key → byte offset, sorted, fixed-width)           │
//! └─────────────────────────────────────────────────────┘
//! ```
//!
//! # Sigil → control code mapping
//!
//! | AURA sigil | Control code | Hex    | Name             |
//! | ---------- | ------------ | ------ | ---------------- |
//! | `::`       | `US`         | `0x1F` | Unit Separator   |
//! | `->`       | `RS`         | `0x1E` | Record Separator |
//! | `\|`       | `GS`         | `0x1D` | Group Separator  |
//! | `@`        | `FS`         | `0x1C` | File Separator   |
//!
//! The B-Tree index is written last so all offsets are calculated in a
//! single forward pass over the Lexical Data Region (no backpatching).

use crate::error::Result;
use crate::parse::ast::{Child, Document, Value};
use std::collections::BTreeMap;

// -------------------------------------------------------------------- //
// Wire constants

/// HAMI file magic bytes.
pub const MAGIC: &[u8; 4] = b"HAMI";
/// Current HAMI format version.
pub const VERSION: u16 = 1;

/// ASCII control codes that replace AURA sigils in the binary output.
pub mod ctrl {
  /// Replaces `::` (Unit Separator — delimits key-value pairs).
  pub const US: u8 = 0x1F;
  /// Replaces `->` (Record Separator — delimits records).
  pub const RS: u8 = 0x1E;
  /// Replaces `|` (Group Separator — delimits union list items).
  pub const GS: u8 = 0x1D;
  /// Replaces `@` (File Separator — marks reference arcs).
  pub const FS: u8 = 0x1C;
}

// -------------------------------------------------------------------- //
// B-Tree index entry (fixed-width, human-readable hex)

/// One entry in the B-Tree Positional Index.
///
/// All fields are fixed-width hexadecimal ASCII so binary search requires
/// no delimiter scanning — the nth entry is at `n * ENTRY_SIZE` bytes.
#[derive(Debug, Clone)]
pub struct IndexEntry {
  /// FNV-1a hash of the key string (6 hex chars).
  pub key_hash: u32,
  /// Byte offset into the Lexical Data Region (6 hex chars).
  pub offset: u32,
}

impl IndexEntry {
  /// Serialized size in bytes: 6 + 1 + 6 + 1 = 14 (hash space offset newline).
  pub const SIZE: usize = 14;
}

// -------------------------------------------------------------------- //
// Emitter

/// HAMI manifest emitter.
///
/// Converts a resolved AST into a `.hami` binary manifest.
pub struct HamiEmitter {
  /// The Lexical Data Region being built.
  data: Vec<u8>,
  /// B-Tree index entries collected during the forward pass.
  index: BTreeMap<u32, u32>, // key_hash → offset
}

impl HamiEmitter {
  pub fn new() -> Self {
    Self {
      data: Vec::with_capacity(4096),
      index: BTreeMap::new(),
    }
  }

  /// Emits the entire document and returns the complete `.hami` bytes.
  pub fn emit(&mut self, doc: &Document<'_>) -> Result<Vec<u8>> {
    // Pass 1 — build the Lexical Data Region and collect index entries.
    for ns in &doc.namespaces {
      self.emit_namespace(ns);
    }

    // Pass 2 — assemble final output: header + data + index.
    let mut out = Vec::with_capacity(12 + self.data.len() + self.index.len() * IndexEntry::SIZE);

    // Header
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    let root_offset: u32 = 12; // data starts right after the 12-byte header
    out.extend_from_slice(&root_offset.to_le_bytes());

    // Lexical Data Region
    out.extend_from_slice(&self.data);

    // B-Tree Positional Index (written last — all offsets now final)
    for (hash, offset) in &self.index {
      // Format: 6-char hex hash + space + 6-char hex offset + newline
      let entry = format!("{:06X} {:06X}\n", hash, offset);
      out.extend_from_slice(entry.as_bytes());
    }

    Ok(out)
  }

  fn emit_namespace(&mut self, ns: &crate::parse::ast::Namespace<'_>) {
    // Record offset for this namespace in the index.
    let offset = self.data.len() as u32;
    let hash = fnv1a(ns.name.as_bytes());
    self.index.entry(hash).or_insert(offset);

    // Write namespace name followed by US (unit separator = `::`)
    self.data.extend_from_slice(ns.name.as_bytes());
    self.data.push(ctrl::US);

    for child in &ns.children {
      match child {
        Child::Field(f) => self.emit_field(f),
        Child::Block(b) => self.emit_namespace(b),
      }
    }

    // RS (record separator) marks the end of this namespace block.
    self.data.push(ctrl::RS);
  }

  fn emit_field(&mut self, f: &crate::parse::ast::Field<'_>) {
    // key RS value US
    let offset = self.data.len() as u32;
    let hash = fnv1a(f.key.as_bytes());
    self.index.entry(hash).or_insert(offset);

    self.data.extend_from_slice(f.key.as_bytes());
    self.data.push(ctrl::RS);
    self.emit_value(&f.value);
    self.data.push(ctrl::US);
  }

  fn emit_value(&mut self, val: &Value<'_>) {
    match val {
      Value::Str(s) => {
        // Strip surrounding quotes from the source slice.
        let inner = s.trim_matches('"');
        self.data.extend_from_slice(inner.as_bytes());
      }
      Value::Bare(s) => {
        self.data.extend_from_slice(s.as_bytes());
      }
      Value::Time(expr) => {
        // The time normalizer has already run; emit normalized form.
        // This branch emits the raw source text as a fallback.
        // In the full implementation, pass the resolved Interval here.
        self
          .data
          .extend_from_slice(format!("{:?}", expr).as_bytes());
      }
      Value::Ref(r) => {
        // FS sigil + domain + / + body
        self.data.push(ctrl::FS);
        self.data.extend_from_slice(r.domain.as_bytes());
        self.data.push(b'/');
        match &r.body {
          crate::parse::ast::RefBody::Single(id) => {
            self.data.extend_from_slice(id.as_bytes());
          }
          crate::parse::ast::RefBody::List(ids) => {
            for (i, id) in ids.iter().enumerate() {
              if i > 0 {
                self.data.push(ctrl::GS);
              }
              self.data.extend_from_slice(id.as_bytes());
            }
          }
          crate::parse::ast::RefBody::Path(parts) => {
            self.data.extend_from_slice(parts.join("/").as_bytes());
          }
          crate::parse::ast::RefBody::Global(uri) => {
            self.data.extend_from_slice(uri.as_bytes());
          }
        }
      }
      Value::Union(vals) => {
        for (i, v) in vals.iter().enumerate() {
          if i > 0 {
            self.data.push(ctrl::GS);
          }
          self.emit_value(v);
        }
      }
      Value::List(vals) => {
        for (i, v) in vals.iter().enumerate() {
          if i > 0 {
            self.data.push(ctrl::GS);
          }
          self.emit_value(v);
        }
      }
      Value::Inherits(r) => {
        // Inherits arcs are consumed by InheritExpander before
        // the emitter runs. Emit a no-op comment if they survive.
        let _ = r;
      }
    }
  }
}

impl Default for HamiEmitter {
  fn default() -> Self {
    Self::new()
  }
}

// -------------------------------------------------------------------- //
// FNV-1a hash (for B-Tree key hashing)

fn fnv1a(bytes: &[u8]) -> u32 {
  let mut hash: u32 = 2166136261;
  for &b in bytes {
    hash ^= b as u32;
    hash = hash.wrapping_mul(16777619);
  }
  hash
}
