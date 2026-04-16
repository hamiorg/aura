//! Schema and directives block processor.
//!
//! Every AURA document opens with a `schema::` block that declares the
//! document kind, language, and annotator. The optional `directives::`
//! block carries compiler and engine hints for this specific file.
//!
//! This module parses both blocks and produces a `FileDirectives` value
//! that the rest of the pipeline consults during compilation.

use crate::error::{CompileError, Result};

/// The declared media kind from the `schema::kind` field.
///
/// Written as `audio::music`, `video::movie`, etc. in AURA source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
  // --- Audio ---
  /// `audio::music` — album, EP, single, or musical work.
  AudioMusic,
  /// `audio::podcast` — podcast episode or show.
  AudioPodcast,
  /// `audio::audiobook` — spoken word with chapters.
  AudioBook,
  /// `audio::live` — live recording.
  AudioLive,

  // --- Video ---
  /// `video::movie` — feature or short film.
  VideoMovie,
  /// `video::series` — episodic series.
  VideoSeries,
  /// `video::podcast` — video podcast episode.
  VideoPodcast,
  /// `video::documentary` — documentary work.
  VideoDoc,
  /// `video::music` — music video.
  VideoMusic,
  /// `video::live` — live performance or concert.
  VideoLive,
  /// `video::short` — short-form content under 10 minutes.
  VideoShort,

  // --- Mixed ---
  /// `mixed::album` — visual album (audio and video tied).
  MixedAlbum,
  /// `mixed::interactive` — interactive or branching media.
  MixedInteractive,

  // --- Special (non-content files) ---
  /// `metadata` — used in `info/metadata.aura`.
  Metadata,
}

impl Kind {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "audio::music" => Some(Self::AudioMusic),
      "audio::album" => Some(Self::AudioMusic), // alias
      "audio::ep" => Some(Self::AudioMusic),    // alias
      "audio::podcast" => Some(Self::AudioPodcast),
      "audio::audiobook" => Some(Self::AudioBook),
      "audio::live" => Some(Self::AudioLive),
      "video::movie" => Some(Self::VideoMovie),
      "video::series" => Some(Self::VideoSeries),
      "video::podcast" => Some(Self::VideoPodcast),
      "video::documentary" => Some(Self::VideoDoc),
      "video::music" => Some(Self::VideoMusic),
      "video::live" => Some(Self::VideoLive),
      "video::short" => Some(Self::VideoShort),
      "mixed::album" => Some(Self::MixedAlbum),
      "mixed::interactive" => Some(Self::MixedInteractive),
      "metadata" => Some(Self::Metadata),
      _ => None,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match self {
      Self::AudioMusic => "audio::music",
      Self::AudioPodcast => "audio::podcast",
      Self::AudioBook => "audio::audiobook",
      Self::AudioLive => "audio::live",
      Self::VideoMovie => "video::movie",
      Self::VideoSeries => "video::series",
      Self::VideoPodcast => "video::podcast",
      Self::VideoDoc => "video::documentary",
      Self::VideoMusic => "video::music",
      Self::VideoLive => "video::live",
      Self::VideoShort => "video::short",
      Self::MixedAlbum => "mixed::album",
      Self::MixedInteractive => "mixed::interactive",
      Self::Metadata => "metadata",
    }
  }
}

/// Parsed contents of the `schema::` and `directives::` blocks.
///
/// Produced once per source file and passed to all downstream pipeline
/// stages so they can adjust behavior based on file-level settings.
#[derive(Debug, Clone)]
pub struct FileDirectives {
  /// The `schema::root` URL (always `https://hami.aduki.org/aura/1.0`
  /// for v0.1 content files).
  pub root: String,
  /// Media kind declared in `schema::kind`.
  pub kind: Kind,
  /// Primary language of this document (IETF BCP 47 tag).
  pub lang: String,
  /// Single annotator ID, if `schema::annotator` is set.
  pub annotator: Option<String>,
  /// Multiple annotator IDs, if `schema::annotators` is set.
  pub annotators: Vec<String>,
  /// `directives::strict` — treat unresolved references as errors.
  pub strict: bool,
  /// `directives::mood-vocabulary` — path or reference to mood vocab.
  pub mood_vocab: Option<String>,
  /// `directives::store` — override store URI for this file.
  pub store: Option<String>,
  /// `directives::variation-default` — default variant when canonical
  /// is unavailable.
  pub variation_default: Option<String>,
}

impl FileDirectives {
  /// Returns `true` if this file declares more than one annotator.
  pub fn has_multiple_annotators(&self) -> bool {
    !self.annotators.is_empty()
  }

  /// Returns the effective annotator list (single or multiple).
  pub fn all_annotators(&self) -> Vec<&str> {
    if self.annotators.is_empty() {
      self.annotator.iter().map(String::as_str).collect()
    } else {
      self.annotators.iter().map(String::as_str).collect()
    }
  }

  /// Validates that all mandatory `schema::` fields are present.
  pub fn validate(&self) -> Result<()> {
    if self.root.is_empty() {
      return Err(CompileError::msg("schema::root is required"));
    }
    if self.lang.is_empty() {
      return Err(CompileError::msg("schema::lang is required"));
    }
    Ok(())
  }
}

impl Default for FileDirectives {
  fn default() -> Self {
    Self {
      root: String::new(),
      kind: Kind::AudioMusic,
      lang: String::new(),
      annotator: None,
      annotators: Vec::new(),
      strict: false,
      mood_vocab: None,
      store: None,
      variation_default: None,
    }
  }
}
