//! Media asset nodes — art, motion covers, and trailers.
//!
//! Art, motion, and trailer assets are uploaded separately to the cloud
//! store to obtain their URL. That URL is stored as literal text in
//! `info/arts.aura`. No binary media files are compiled into `.atom` or
//! `.hami` outputs — only the URL references.
//!
//! # ATOM integration
//!
//! | Type          | ATOM class | Where stored          |
//! | ------------- | ---------- | --------------------- |
//! | `ArtNode`     | `0x15`     | `.hami` manifest only |
//! | `MotionNode`  | `0x16`     | `.hami` manifest only |
//! | `TrailerNode` | `0x17`     | `.hami` manifest only |
//!
//! None of these are interval-tree indexed — they carry no temporal data.

use crate::person::StringRef;

/// Recognized aspect ratios for art assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
  /// 1:1 — album cover, podcast cover, Apple Music art
  Square = 0x01,
  /// Generic wide — YouTube thumbnail, Spotify header
  Landscape = 0x02,
  /// Generic tall — movie poster (3:4 approximate)
  Portrait = 0x03,
  /// 16:9 — video thumbnail, TV series banner
  W16H9 = 0x04,
  /// 4:3 — legacy video, some TV formats
  W4H3 = 0x05,
  /// 9:16 — vertical mobile full-bleed, portrait video
  W9H16 = 0x06,
  /// 21:9 — ultrawide cinematic banner
  W21H9 = 0x07,
  /// 2:3 — tall movie poster (international standard)
  W2H3 = 0x08,
  /// Non-standard ratio with explicit dimensions
  Custom = 0xFF,
}

impl Ratio {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "square" => Some(Self::Square),
      "landscape" => Some(Self::Landscape),
      "portrait" => Some(Self::Portrait),
      "16:9" => Some(Self::W16H9),
      "4:3" => Some(Self::W4H3),
      "9:16" => Some(Self::W9H16),
      "21:9" => Some(Self::W21H9),
      "2:3" => Some(Self::W2H3),
      "custom" => Some(Self::Custom),
      _ => None,
    }
  }
}

/// A static cover art or image asset.
///
/// ATOM class `0x15`. Stored in `.hami` manifests only.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ArtNode {
  /// Generated ID with `ar` prefix, e.g. `"ar4xab3c"`.
  pub id: [u8; 8],
  /// Aspect ratio of this asset.
  pub ratio: Ratio,
  /// Cloud CDN URL — not a local file path.
  pub url: StringRef,
  /// Optional editorial note.
  pub note: Option<StringRef>,
}

/// The kind of motion cover asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MotionKind {
  AlbumMotion = 0x01,
  EpisodeMotion = 0x02,
  MovieMotion = 0x03,
  ShowMotion = 0x04,
  Background = 0x05,
  Custom = 0xFF,
}

/// An animated motion cover or looping video asset.
///
/// Typically 3–30 seconds. Apple Music-style animated album covers.
/// ATOM class `0x16`. Stored in `.hami` manifests only.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MotionNode {
  /// Generated ID with `mo` prefix, e.g. `"mo7xk9p2"`.
  pub id: [u8; 8],
  /// Motion cover kind.
  pub kind: MotionKind,
  /// Cloud CDN URL (HLS or DASH manifest for adaptive streaming).
  pub url: StringRef,
  /// Duration in seconds.
  pub duration: f32,
  /// `true` = loops (album-motion), `false` = plays once (background).
  pub loops: bool,
  /// Aspect ratio.
  pub ratio: Ratio,
}

/// The kind of trailer or preview clip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrailerKind {
  MovieTrailer = 0x01,
  EpisodeTrailer = 0x02,
  PodcastTrailer = 0x03,
  SeriesTrailer = 0x04,
  Teaser = 0x05,
  Announcement = 0x06,
  BehindTheScenes = 0x07,
  Custom = 0xFF,
}

/// A promotional trailer or preview clip.
///
/// Inherits all `MotionNode` fields; typically longer (30s–3min),
/// non-looping, with deliberate editorial structure.
/// ATOM class `0x17`. Stored in `.hami` manifests only.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrailerNode {
  /// Generated ID with `tr` prefix, e.g. `"tr6xp3lm"`.
  pub id: [u8; 8],
  /// Trailer kind.
  pub kind: TrailerKind,
  /// Cloud CDN URL.
  pub url: StringRef,
  /// Duration in seconds.
  pub duration: f32,
  /// `true` = loops, `false` = plays once (usual for trailers).
  pub loops: bool,
  /// Aspect ratio.
  pub ratio: Ratio,
  /// Optional ISO 8601 release date as a Unix date (days since epoch).
  pub released: Option<u32>,
}
