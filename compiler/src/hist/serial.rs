//! TOML-serializable mirror types for the history object store.
//!
//! `core::delta::TakeObject` and `SourceDelta` are kept free of any
//! external dependencies. These mirror types add `serde` derives so the
//! compiler can serialize/deserialize to TOML without touching `core`.
//!
//! # On-disk TOML format
//!
//! ```toml
//! id        = "tx3ab7k"
//! parent    = "tx3ab3c"   # absent for origin takes
//! stream    = "main"
//! timestamp = 1713276000
//! message   = "first complete draft"
//!
//! [[deltas]]
//! op   = "upsert"
//! path = "verse/one/line/one"
//! aura = '''
//! verse/one/line/one::
//!   text -> "The signal fades"
//!   time -> 22s~1m10s
//! '''
//!
//! [[deltas]]
//! op   = "drop"
//! path = "bridge/two"
//! ```
//!
//! TOML multiline literal strings (`'''`) handle AURA content blocks
//! without requiring any escaping of `"` or `\` characters.

use aura::delta::{SourceDelta, TakeObject};
use serde::{Deserialize, Serialize};

/// TOML-serializable mirror of `core::delta::TakeObject`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeToml {
  pub id: String,
  pub parent: Option<String>,
  pub stream: String,
  pub timestamp: u64,
  pub message: Option<String>,
  #[serde(default)]
  pub deltas: Vec<DeltaToml>,
}

/// TOML-serializable mirror of `core::delta::SourceDelta`.
///
/// Uses a `tag = "op"` for the `op` discriminant field so the TOML looks
/// like `op = "upsert"` / `op = "drop"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum DeltaToml {
  Upsert { path: String, aura: String },
  Drop { path: String },
}

// -------------------------------------------------------------------- //
// Conversions between core types and TOML mirror types

impl From<TakeObject> for TakeToml {
  fn from(t: TakeObject) -> Self {
    Self {
      id: t.id,
      parent: t.parent,
      stream: t.stream,
      timestamp: t.timestamp,
      message: t.message,
      deltas: t.deltas.into_iter().map(DeltaToml::from).collect(),
    }
  }
}

impl From<TakeToml> for TakeObject {
  fn from(t: TakeToml) -> Self {
    Self {
      id: t.id,
      parent: t.parent,
      stream: t.stream,
      timestamp: t.timestamp,
      message: t.message,
      deltas: t.deltas.into_iter().map(SourceDelta::from).collect(),
    }
  }
}

impl From<SourceDelta> for DeltaToml {
  fn from(d: SourceDelta) -> Self {
    match d {
      SourceDelta::Upsert { path, aura } => Self::Upsert { path, aura },
      SourceDelta::Drop { path } => Self::Drop { path },
    }
  }
}

impl From<DeltaToml> for SourceDelta {
  fn from(d: DeltaToml) -> Self {
    match d {
      DeltaToml::Upsert { path, aura } => Self::Upsert { path, aura },
      DeltaToml::Drop { path } => Self::Drop { path },
    }
  }
}
