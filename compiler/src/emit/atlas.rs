//! ATLAS DTW alignment file emitter — produces `.atlas` files.
//!
//! An `.atlas` file maps timestamps from a canonical stream to a
//! variant stream using a DTW (Dynamic Time Warping) warp path.
//!
//! # `.atlas` file layout
//!
//! ```text
//! ┌────────────────────────────────────────────────────────┐
//! │  Magic: "ATLS" (4 bytes)                               │
//! │  Version: u16                                          │
//! │  Source ID: [u8; 8]   (canonical track/episode ID)    │
//! │  Target ID: [u8; 8]   (variant ID)                     │
//! │  Point count: u32                                      │
//! ├────────────────────────────────────────────────────────┤
//! │  WarpPoint[0]  { source_t: f32, target_t: f32 }        │
//! │  WarpPoint[1]  { ... }                                 │
//! │  WarpPoint[N]  { ... }                                 │
//! └────────────────────────────────────────────────────────┘
//! ```
//!
//! Each `WarpPoint` is 8 bytes (two f32s). The array is ordered by
//! `source_t` for binary search during variant playback.

use crate::error::Result;
use aura::node::WarpPoint;

pub const MAGIC: &[u8; 4] = b"ATLS";
pub const VERSION: u16 = 1;

/// Header size: 4 (magic) + 2 (version) + 8 (source_id) + 8 (target_id)
///            + 4 (count) = 26 bytes.
pub const HEADER_SIZE: usize = 26;

/// An alignment specification passed to the `AtlasEmitter`.
pub struct AlignSpec {
  /// 8-byte canonical source ID (e.g. track `t7xab3c\0`).
  pub source_id: [u8; 8],
  /// 8-byte variant target ID (e.g. variant `v3qr7st\0`).
  pub target_id: [u8; 8],
  /// The DTW warp path computed by the alignment algorithm.
  pub warp: Vec<WarpPoint>,
}

/// ATLAS alignment file emitter.
pub struct AtlasEmitter;

impl AtlasEmitter {
  pub fn new() -> Self {
    Self
  }

  /// Emits an `.atlas` file from a pre-computed alignment spec.
  pub fn emit(&self, spec: &AlignSpec) -> Result<Vec<u8>> {
    let count = spec.warp.len() as u32;
    let mut out = Vec::with_capacity(HEADER_SIZE + spec.warp.len() * 8);

    // Header
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&spec.source_id);
    out.extend_from_slice(&spec.target_id);
    out.extend_from_slice(&count.to_le_bytes());

    // Warp path
    for pt in &spec.warp {
      out.extend_from_slice(&pt.source_t.to_le_bytes());
      out.extend_from_slice(&pt.target_t.to_le_bytes());
    }

    Ok(out)
  }
}

impl Default for AtlasEmitter {
  fn default() -> Self {
    Self::new()
  }
}
