//! Industry entity nodes — production studios and record labels.
//!
//! Both support an ownership/imprint hierarchy via the `parent` field:
//! a subsidiary studio is owned by a parent studio; an imprint is
//! owned by a parent label. The engine traverses these arcs for rights
//! and credits resolution.
//!
//! # ATOM integration
//!
//! | Type         | ATOM class | Where stored           |
//! | ------------ | ---------- | ---------------------- |
//! | `StudioNode` | `0x18`     | `.hami` manifest only  |
//! | `LabelNode`  | `0x19`     | `.hami` manifest only  |

use crate::person::StringRef;

/// The kind of production studio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StudioKind {
  Film = 0x01,
  Television = 0x02,
  Animation = 0x03,
  Documentary = 0x04,
  Music = 0x05,
  Game = 0x06,
  Custom = 0xFF,
}

/// A production studio, production company, or broadcast network.
///
/// Defined in `info/studios.aura`. Referenced via `@studio/id`.
/// ATOM class `0x18`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StudioNode {
  /// Generated ID with `st` prefix, e.g. `"st4xab3c"`.
  pub id: [u8; 8],
  /// Full legal studio name → string pool.
  pub name: StringRef,
  /// Studio kind.
  pub kind: StudioKind,
  /// ISO 3166-1 alpha-2 country code.
  pub country: [u8; 2],
  /// Parent studio ID for ownership hierarchy, if any.
  /// Trailing bytes zeroed when absent.
  pub parent: Option<[u8; 8]>,
  /// `@art/id` reference to studio logo — raw ID bytes.
  pub logo: Option<[u8; 8]>,
}

/// The kind of record label or publishing imprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LabelKind {
  Major = 0x01,
  Independent = 0x02,
  Imprint = 0x03,
  Publisher = 0x04,
  Distributor = 0x05,
  Custom = 0xFF,
}

/// A record label or publishing imprint.
///
/// Defined in `info/labels.aura`. Referenced via `@label/id`.
/// ATOM class `0x19`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LabelNode {
  /// Generated ID with `lb` prefix, e.g. `"lb7mn4rp"`.
  pub id: [u8; 8],
  /// Full legal label name → string pool.
  pub name: StringRef,
  /// Label kind.
  pub kind: LabelKind,
  /// ISO 3166-1 alpha-2 country code.
  pub country: [u8; 2],
  /// Parent label ID for imprint hierarchy, if any.
  pub parent: Option<[u8; 8]>,
}
