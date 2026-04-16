//! Vocabulary nodes — genre, role, and mood slugs.
//!
//! Vocabulary nodes are the one exception to the generated hex ID rule.
//! They use slug IDs — lowercase words with hyphens for compounds.
//! Slugs are stable, human-readable, and platform-canonical.
//!
//! Examples: `electronic`, `afro-soul`, `main-artist`, `ethereal`
//!
//! Resolution cascade:
//! 1. Local `meta/` folder for this project
//! 2. Parent catalog's `meta/` folder
//! 3. Global platform vocabulary at `@aduki.org/genre/`, `…/role/`, `…/mood/`
//! 4. Unresolved → stored as string literal, compile warning in strict mode

/// The kind of vocabulary a `VocabNode` represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VocabKind {
  /// Genre classification (e.g. `electronic`, `afro-soul`).
  Genre,
  /// Contributor role (e.g. `main-artist`, `lead-vocal`).
  Role,
  /// Emotional or tonal descriptor (e.g. `ethereal`, `melancholic`).
  Mood,
}

/// A first-class vocabulary node compiled from `meta/genres.aura`,
/// `meta/roles.aura`, or `meta/moods.aura`.
///
/// ATOM node class: `0x0C`.
#[derive(Debug, Clone)]
pub struct VocabNode {
  /// Slug ID, e.g. `"afro-soul"`, `"main-artist"`, `"ethereal"`.
  pub slug: String,
  /// Whether this is a genre, role, or mood node.
  pub kind: VocabKind,
  /// Human-readable name, e.g. `"Afro-Soul"`.
  pub name: String,
  /// Optional parent slug for genre/role hierarchies.
  /// E.g. `afro-soul` has parent `soul`, `soul` has parent `rnb`.
  pub parent: Option<String>,
  /// Optional region tag (primarily used for genre nodes).
  pub region: Option<String>,
  /// Optional editorial note.
  pub note: Option<String>,
}

impl VocabNode {
  /// Returns the domain string used in `@genre/`, `@role/`, or `@mood/`
  /// references.
  pub fn domain(&self) -> &str {
    match self.kind {
      VocabKind::Genre => "genre",
      VocabKind::Role => "role",
      VocabKind::Mood => "mood",
    }
  }
}
