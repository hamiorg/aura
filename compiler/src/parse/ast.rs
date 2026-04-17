//! Abstract Syntax Tree node types.
//!
//! The parser produces a tree of `ASTNode` values that mirrors the AURA
//! document hierarchy: documents contain namespaces, namespaces contain
//! fields or sub-nodes, fields carry values or nested namespaces.
//!
//! # AURA hierarchy
//!
//! ```text
//! Document
//!   └── Namespace ("manifest::", "verse/one::", "$live::", ...)
//!         ├── Field  (key -> value)
//!         ├── Field  (key -> reference)
//!         └── Namespace (nested block)
//! ```
//!
//! Nodes are identified by slash-identifiers (`verse/one`, `chorus/two`)
//! which the diff engine uses for node-level history tracking.
//!
//! # Vocab slug escaping
//!
//! Namespace blocks prefixed with `$` (e.g. `$live::`, `$dark::`) are
//! raw vocabulary slugs. The `$` is stored in `name` but stripped when
//! resolving the actual slug. `raw_slug = true` disables W006 checking.

use crate::error::Span;
use aura::interval::Interval;

// -------------------------------------------------------------------- //
// Top-level document

/// A parsed AURA document.
///
/// Corresponds to one `.aura` source file. Contains an ordered sequence
/// of top-level namespace blocks.
#[derive(Debug, Clone)]
pub struct Document<'src> {
  /// Top-level namespace blocks in source order.
  pub namespaces: Vec<Namespace<'src>>,
  /// Source file path (for diagnostics).
  pub path: Option<std::path::PathBuf>,
}

// -------------------------------------------------------------------- //
// Namespace (block delimited by `::`)

/// A namespace block — the top-level structural unit of an AURA document.
///
/// Opened by `name::` and contains an indented sequence of fields and
/// nested sub-namespaces.
///
/// Examples:
/// - `manifest::` — collection manifest
/// - `verse/one::` — first verse (content node)
/// - `support::` — container for all support nodes
/// - `$live::` — raw vocab slug block (W006 skipped inside)
#[derive(Debug, Clone)]
pub struct Namespace<'src> {
  /// The name before `::`, e.g. `"manifest"`, `"verse/one"`, `"$live"`.
  pub name: &'src str,
  /// Slash-identifier path for this node in the AST, e.g.
  /// `"verse/one"` or `"support/segments/intro/one"`.
  pub path: String,
  /// Fields and nested namespaces inside this block.
  pub children: Vec<Child<'src>>,
  /// Source location of the opening `name::` token.
  pub span: Span,
  /// Node type inferred from the name (e.g. `verse`, `chorus`, `scene`).
  pub node_type: NodeType,
  /// `true` if this block was opened with `$name::` — a raw vocabulary slug.
  /// W006 key-checking is skipped for all fields within raw-slug blocks.
  pub raw_slug: bool,
}

/// A child item inside a namespace block.
#[derive(Debug, Clone)]
pub enum Child<'src> {
  Field(Field<'src>),
  Block(Namespace<'src>),
}

// -------------------------------------------------------------------- //
// Field (key -> value)

/// A key-value field inside a namespace block.
///
/// Examples:
/// - `name -> "Signal Loss"`
/// - `creator -> @person/p4xt9k2`
/// - `time -> 22s~1m10s`
/// - `genre -> Electronic | Afro-Soul`
#[derive(Debug, Clone)]
pub struct Field<'src> {
  /// The key string, e.g. `"name"`, `"time"`, `"cleared"`.
  pub key: &'src str,
  /// The value on the right-hand side of `->`.
  pub value: Value<'src>,
  /// Whether the field is marked `!` (required) or `?` (optional).
  pub marker: Option<FieldMarker>,
  /// Source location of the key.
  pub span: Span,
}

/// Optional or required field marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldMarker {
  /// `!` — field must be present or compilation fails.
  Required,
  /// `?` — field may be absent; engine skips gracefully.
  Optional,
  /// `%` — custom key: intentionally outside the standard AURA vocabulary.
  /// W006 key-checking is suppressed for this field.
  /// Usage: `key % -> value`  (space before `%` is conventional).
  Custom,
}

// -------------------------------------------------------------------- //
// Value

/// The value on the right-hand side of a `->` field assignment.
#[derive(Debug, Clone)]
pub enum Value<'src> {
  /// A quoted string literal, e.g. `"Signal Loss"`.
  Str(&'src str),
  /// A bare identifier or number, e.g. `live`, `1.0.0`, `explicit`.
  Bare(&'src str),
  /// A time expression: range, offset, triple, or point anchor.
  Time(TimeExpr<'src>),
  /// An `@domain/id` reference.
  Ref(Reference<'src>),
  /// A `|`-separated union of values or references.
  Union(Vec<Value<'src>>),
  /// A `[...]` list of references or a time triple.
  List(Vec<Value<'src>>),
  /// An inherits arc `>> @info/metadata`.
  Inherits(Reference<'src>),
}

// -------------------------------------------------------------------- //
// Time expressions (before normalization)

/// A time expression as written by the author, before normalization to
/// `Interval`. The `time::TimeNorm` pass converts these to `Interval`.
#[derive(Debug, Clone)]
pub enum TimeExpr<'src> {
  /// `start~end` — range syntax. Duration is derived.
  Range { start: &'src str, end: &'src str },
  /// `start+duration` — offset syntax. End is derived.
  Offset { start: &'src str, dur: &'src str },
  /// `[start, end, duration]` — explicit triple.
  Triple {
    start: &'src str,
    end: &'src str,
    dur: &'src str,
  },
  /// `@time/value` — point anchor.
  Anchor(&'src str),
}

/// A resolved time value: the normalized `Interval` with the original
/// source expression preserved for error messages.
#[derive(Debug, Clone)]
pub struct ResolvedTime<'src> {
  pub interval: Interval,
  pub source: TimeExpr<'src>,
}

// -------------------------------------------------------------------- //
// References

/// An `@domain/id` or `@domain/[id1, id2]` reference in AURA source.
#[derive(Debug, Clone)]
pub struct Reference<'src> {
  /// The domain, e.g. `"person"`, `"track"`, `"genre"`, `"time"`.
  pub domain: &'src str,
  /// The body after the domain slash.
  pub body: RefBody<'src>,
  /// Source location of the `@` sigil.
  pub span: Span,
}

/// The body of a reference — scalar ID, list of IDs, or a path.
#[derive(Debug, Clone)]
pub enum RefBody<'src> {
  /// `@domain/id` — singular reference.
  Single(&'src str),
  /// `@domain/[id1, id2, ...]` — plural list of IDs.
  List(Vec<&'src str>),
  /// `@domain/path/sub/path` — node path reference (e.g. `@verse/one/line/three`).
  Path(Vec<&'src str>),
  /// `@aduki.org/...` — global cloud reference.
  Global(&'src str),
}

// -------------------------------------------------------------------- //
// Node type classification

/// The type of a namespace block, inferred from its name.
///
/// This is used by the emitter to assign the correct ATOM `node_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
  // --- Project entry / folder index ---
  /// `name::` — the project entry identifier block in `name.aura`.
  Name,

  // --- Schema/manifest (non-interval) ---
  Schema,
  Manifest,
  Directives,
  Exports,
  Namespace,
  Collection,
  Members,
  Seasons,
  Credits,
  Links,
  Related,
  Availability,
  Info,
  Meta,

  // --- Collection sub-containers ---
  /// `tracks::` — container for track member list in a manifest.
  Tracks,
  /// `episodes::` — container for episode member list.
  Episodes,
  /// `scenes::` — container for scene member list.
  Scenes,
  /// `variants::` — container for variant list.
  Variants,
  /// `contains::` — folder entry index in `name.aura` sub-folder files.
  /// Keys inside are generated AURA IDs, not vocabulary keys; W006 is suppressed.
  Contains,

  // --- Content nodes (interval-indexed) ---
  Act,
  Scene,
  Shot,
  Verse,
  Chorus,
  Bridge,
  Intro,
  Outro,
  Hook,
  Drop,
  Interlude,
  Breakdown,
  PreChorus,
  PostChorus,
  Chapter,
  Segment,
  Section,
  Line,
  Dialogue,
  Word,
  Token,
  Syllable,
  Phoneme,
  Letter,
  Character,

  // --- Support nodes (interval-indexed) ---
  Support,
  Segments,
  Instruments,
  Chapters,
  Windows,
  Translations,
  Moods,
  Rights,
  Slots,
  Anchors,
  Tempo,
  Samples,
  Explainers,
  Interpolations,
  Instructions,
  Events,

  // --- People and vocabulary ---
  People,
  Annotators,
  Genres,
  Roles,
  MoodsVocab,

  // --- Media and industry ---
  Arts,
  Motions,
  Trailers,
  Studios,
  Labels,
  Watch,
  Buy,
  Rent,
  Download,

  /// Vocab slug — a `$name::` block inside a vocab container.
  VocabSlug,

  /// Unknown — the name didn't match any known type.
  Unknown,
}

impl NodeType {
  /// Infers the node type from the first segment of a namespace name.
  ///
  /// E.g. `"verse/one"` → `NodeType::Verse`,
  ///       `"support"` → `NodeType::Support`,
  ///       `"$live"` → `NodeType::VocabSlug`.
  pub fn from_name(name: &str) -> Self {
    // `$`-prefixed names are always vocab slugs.
    if name.starts_with('$') {
      return Self::VocabSlug;
    }
    let segment = name.split('/').next().unwrap_or(name);
    match segment {
      "name" => Self::Name,
      "schema" => Self::Schema,
      "manifest" => Self::Manifest,
      "directives" => Self::Directives,
      "exports" => Self::Exports,
      "namespace" => Self::Namespace,
      "collection" => Self::Collection,
      "members" => Self::Members,
      "seasons" => Self::Seasons,
      "credits" => Self::Credits,
      "links" => Self::Links,
      "related" => Self::Related,
      "availability" => Self::Availability,
      "info" => Self::Info,
      "meta" => Self::Meta,
      "tracks" => Self::Tracks,
      "episodes" => Self::Episodes,
      "scenes" => Self::Scenes,
      "variants" => Self::Variants,
      "contains" => Self::Contains,
      "act" => Self::Act,
      "scene" => Self::Scene,
      "shot" => Self::Shot,
      "verse" => Self::Verse,
      "chorus" => Self::Chorus,
      "bridge" => Self::Bridge,
      "intro" => Self::Intro,
      "outro" => Self::Outro,
      "hook" => Self::Hook,
      "drop" => Self::Drop,
      "interlude" => Self::Interlude,
      "breakdown" => Self::Breakdown,
      "pre-chorus" => Self::PreChorus,
      "post-chorus" => Self::PostChorus,
      "chapter" => Self::Chapter,
      "segment" => Self::Segment,
      "section" => Self::Section,
      "line" => Self::Line,
      "dialogue" => Self::Dialogue,
      "word" => Self::Word,
      "token" => Self::Token,
      "syllable" => Self::Syllable,
      "phoneme" => Self::Phoneme,
      "letter" => Self::Letter,
      "character" => Self::Character,
      "support" => Self::Support,
      "segments" => Self::Segments,
      "instruments" => Self::Instruments,
      "chapters" => Self::Chapters,
      "windows" => Self::Windows,
      "translations" => Self::Translations,
      "moods" => Self::Moods,
      "rights" => Self::Rights,
      "slots" => Self::Slots,
      "anchors" => Self::Anchors,
      "tempo" => Self::Tempo,
      "samples" => Self::Samples,
      "explainers" => Self::Explainers,
      "interpolations" => Self::Interpolations,
      "instructions" => Self::Instructions,
      "events" => Self::Events,
      "people" | "persons" | "authors" => Self::People,
      "annotators" => Self::Annotators,
      "genres" => Self::Genres,
      "roles" => Self::Roles,
      "arts" => Self::Arts,
      "motions" => Self::Motions,
      "trailers" => Self::Trailers,
      "studios" => Self::Studios,
      "labels" => Self::Labels,
      "watch" => Self::Watch,
      "buy" => Self::Buy,
      "rent" => Self::Rent,
      "download" => Self::Download,
      _ => Self::Unknown,
    }
  }

  /// Returns `true` if this node type is interval-indexed in `.atom`.
  pub fn is_interval(&self) -> bool {
    matches!(
      self,
      Self::Act
        | Self::Scene
        | Self::Shot
        | Self::Verse
        | Self::Chorus
        | Self::Bridge
        | Self::Intro
        | Self::Outro
        | Self::Hook
        | Self::Drop
        | Self::Interlude
        | Self::Breakdown
        | Self::PreChorus
        | Self::PostChorus
        | Self::Chapter
        | Self::Segment
        | Self::Section
        | Self::Line
        | Self::Dialogue
        | Self::Word
        | Self::Token
        | Self::Syllable
        | Self::Phoneme
        | Self::Letter
        | Self::Character
        | Self::Moods
        | Self::Rights
        | Self::Slots
        | Self::Anchors
        | Self::Tempo
        | Self::Events
    )
  }
}
