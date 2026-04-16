//! Interval triple — the canonical time representation everywhere in AURA.
//!
//! All time expressions in AURA source normalize to this triple before
//! emission. The invariant `low + duration == high` is enforced by the
//! compiler and validated at query time by the engine.
//!
//! # Time syntax → Interval mapping
//!
//! | AURA source          | low  | high | duration |
//! | -------------------- | ---- | ---- | -------- |
//! | `22s~1m10s`          | 22.0 | 70.0 | 48.0     |
//! | `22s+48s`            | 22.0 | 70.0 | 48.0     |
//! | `[22s, 1m10s, 48s]`  | 22.0 | 70.0 | 48.0     |
//! | `@time/1m32s`        | 92.0 | 92.0 | 0.0      |
//!
//! Point anchors have `low == high` and `duration == 0.0`.

/// A three-value time triple stored in `.atom` for every temporal object.
///
/// All three fields are always written. The engine validates
/// `low + duration == high`. If any two are provided and the third is
/// absent, the compiler derives and writes the third value.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
  /// Interval start, in seconds from the beginning of the media.
  pub low: f32,
  /// Interval end, in seconds. Equals `low` for point anchors.
  pub high: f32,
  /// Pre-computed duration `high - low`. Zero for point anchors.
  pub duration: f32,
}

impl Interval {
  /// Creates a new interval from start and end, deriving duration.
  pub fn from_range(low: f32, high: f32) -> Self {
    Self {
      low,
      high,
      duration: high - low,
    }
  }

  /// Creates an interval from start and duration, deriving end.
  pub fn from_start_dur(low: f32, duration: f32) -> Self {
    Self {
      low,
      high: low + duration,
      duration,
    }
  }

  /// Creates an explicit triple. Returns `Err` if the invariant is violated.
  pub fn from_triple(low: f32, high: f32, duration: f32) -> Result<Self, InvariantError> {
    let derived = high - low;
    // Allow a small floating-point tolerance.
    if (derived - duration).abs() > 1e-4 {
      return Err(InvariantError {
        low,
        high,
        duration,
      });
    }
    Ok(Self {
      low,
      high,
      duration,
    })
  }

  /// Creates a point anchor at a single instant. Duration is 0.
  pub fn point(t: f32) -> Self {
    Self {
      low: t,
      high: t,
      duration: 0.0,
    }
  }

  /// Returns `true` if this is a point anchor (zero duration).
  pub fn is_point(&self) -> bool {
    self.duration == 0.0 && (self.low - self.high).abs() < 1e-6
  }
}

/// Error returned when the time invariant `low + duration == high` is violated.
#[derive(Debug)]
pub struct InvariantError {
  pub low: f32,
  pub high: f32,
  pub duration: f32,
}

impl std::fmt::Display for InvariantError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "time invariant violated: low({}) + duration({}) != high({}) (diff: {})",
      self.low,
      self.duration,
      self.high,
      (self.low + self.duration - self.high).abs()
    )
  }
}

impl std::error::Error for InvariantError {}
