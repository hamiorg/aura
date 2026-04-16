//! AtomNode and HamiNode — the fundamental binary data structures.
//!
//! `AtomNode` is the unit of the `.atom` flat-array interval tree.
//! `HamiNode` represents a manifest entry in a `.hami` B-Tree index.
//!
//! All structs are `#[repr(C)]` so the compiler and engine share
//! identical memory layouts with zero-copy mmap casting.

/// Node class byte values embedded in every `AtomNode`.
///
/// Stabbing queries accept an optional bitmask filter that maps to
/// these values. The filter is applied inside the SIMD evaluation
/// loop at no additional per-node cost.
pub mod class {
  pub const CONTENT: u32 = 0x01;
  pub const SEGMENT: u32 = 0x02;
  pub const INSTRUMENT: u32 = 0x03;
  pub const CHAPTER: u32 = 0x04;
  pub const CREDIT: u32 = 0x05;
  pub const TRANSLATION: u32 = 0x06;
  pub const MOOD: u32 = 0x07;
  pub const RIGHTS: u32 = 0x08;
  pub const SLOT: u32 = 0x09;
  pub const ANCHOR: u32 = 0x0A;
  pub const ANNOTATOR: u32 = 0x0B;
  pub const VOCAB: u32 = 0x0C;
  pub const EVENT: u32 = 0x0D;
  pub const TEMPO: u32 = 0x0E;
  pub const SAMPLE: u32 = 0x0F;
  pub const EXPLAINER: u32 = 0x10;
  pub const INTERPOLATION: u32 = 0x11;
  pub const INSTRUCTION: u32 = 0x12;
  pub const ACCESS: u32 = 0x13;
  pub const HISTORY: u32 = 0x14;
  pub const ART: u32 = 0x15;
  pub const MOTION: u32 = 0x16;
  pub const TRAILER: u32 = 0x17;
  pub const STUDIO: u32 = 0x18;
  pub const LABEL: u32 = 0x19;
  pub const WATCH: u32 = 0x1A;
  pub const BUY: u32 = 0x1B;
  pub const RENT: u32 = 0x1C;
  pub const DOWNLOAD: u32 = 0x1D;
}

/// The fundamental unit of the `.atom` flat-array interval tree.
///
/// Six contiguous 32-bit fields. Size: 24 bytes.
/// One AVX-2 register (256-bit) processes ~10 AtomNodes per cycle.
/// The SIMD loop evaluates 8-node blocks, covering `low`, `high`,
/// and `duration` of two adjacent nodes in a single CPU cycle.
///
/// # Memory layout
/// ```text
/// offset  field       size
/// 0       low         4 bytes   interval start (seconds)
/// 4       high        4 bytes   interval end (seconds)
/// 8       duration    4 bytes   high - low (pre-computed)
/// 12      max         4 bytes   max high in subtree (augmented)
/// 16      data_ptr    4 bytes   byte offset into .hami companion
/// 20      node_class  4 bytes   class byte (0x01 content … 0x1D download)
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtomNode {
  /// Interval start in seconds.
  pub low: f32,
  /// Interval end in seconds.
  pub high: f32,
  /// Pre-computed duration: `high - low`. Stored explicitly so the
  /// SIMD loop never subtracts per stabbing query.
  pub duration: f32,
  /// Maximum `high` value in this node's subtree. Enables O(1) subtree
  /// pruning: if `t > max` the entire subtree is skipped.
  pub max: f32,
  /// Byte offset into the companion `.hami` file where this node's
  /// key-value data starts.
  pub data_ptr: u32,
  /// Node class byte — identifies content type for bitmask filtering.
  /// See the `class` module for constants.
  pub node_class: u32,
}

impl AtomNode {
  /// Creates a new node. `max` starts equal to `high`; the emitter
  /// updates it in a second pass over the flat array.
  pub fn new(low: f32, high: f32, data_ptr: u32, node_class: u32) -> Self {
    Self {
      low,
      high,
      duration: high - low,
      max: high,
      data_ptr,
      node_class,
    }
  }

  /// Returns `true` if this is a point anchor (zero duration).
  pub fn is_point(&self) -> bool {
    self.duration == 0.0 && (self.low - self.high).abs() < 1e-6
  }
}

/// A manifest entry in a `.hami` B-Tree index.
///
/// HamiNode is not interval-tree indexed — it lives in the Lexical Data
/// Region and is addressed by B-Tree key lookup, not by timestamp.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct HamiNode {
  /// The B-Tree key: a hash of the AURA key string.
  pub key_hash: u32,
  /// Byte offset into the Lexical Data Region where this node's
  /// key-value pair starts.
  pub offset: u32,
  /// Node class (same constants as AtomNode) for typed filtering.
  pub node_class: u32,
}

/// A WarpPoint maps a source timestamp to a target timestamp in a
/// `.atlas` DTW alignment file.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WarpPoint {
  /// Timestamp in the canonical (source) stream, seconds.
  pub source_t: f32,
  /// Corresponding timestamp in the variant (target) stream, seconds.
  pub target_t: f32,
}
