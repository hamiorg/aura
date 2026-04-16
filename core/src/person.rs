//! PersonNode and AnnotatorNode — human entity types.
//!
//! All human entities in AURA — artists, directors, narrators,
//! annotators — are people first. They are distinguished by which
//! namespace they are indexed in: `info/people.aura` for content
//! contributors, `info/annotators.aura` for documentation authors.
//!
//! Both use the `p` prefix for generated IDs. The engine distinguishes
//! them by reference domain (`@person/` vs `@annotator/`), not by ID
//! format.

/// Byte reference into a `.hami` string pool.
/// Points to a null-terminated UTF-8 string at the given offset.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringRef {
  /// Byte offset into the `.hami` string pool region.
  pub offset: u32,
  /// Byte length of the string (not including null terminator).
  pub len: u16,
}

/// What kind of contributor this person is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PersonKind {
  Artist = 0x01,
  Actor = 0x02,
  Director = 0x03,
  Host = 0x04,
  Narrator = 0x05,
  Composer = 0x06,
  Producer = 0x07,
  Writer = 0x08,
  Editor = 0x09,
  Engineer = 0x0A,
  Other = 0xFF,
}

/// A content contributor — artist, actor, director, host, narrator, etc.
///
/// Defined in `info/people.aura`. Referenced via `@person/id`,
/// `@author/id`, `@people/[a, b]`, or `@authors/[a, b]`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PersonNode {
  /// Generated ID, e.g. `"p4xt9k2"`. Fixed 7–8 bytes with `p` prefix.
  pub id: [u8; 8],
  /// Given name → string pool reference.
  pub first: StringRef,
  /// Middle name(s) — optional.
  pub middle: Option<StringRef>,
  /// Family name — optional for mononyms.
  pub last: Option<StringRef>,
  /// Short on-screen label for captions and mini-players.
  pub screen: Option<StringRef>,
  /// Legal name, e.g. `"Mario A. Mwangi"`.
  pub legal: Option<StringRef>,
  /// Primary role kind for this person.
  pub kind: PersonKind,
  /// ISO 3166-1 alpha-2 country code, e.g. `b"KE"`.
  pub country: [u8; 2],
}

/// Bitfield of annotator role flags.
///
/// An annotator may hold more than one role simultaneously.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnotatorRoles(pub u8);

impl AnnotatorRoles {
  pub const TRANSCRIBER: u8 = 0x01;
  pub const EDITOR: u8 = 0x02;
  pub const TRANSLATOR: u8 = 0x04;
  pub const ANNOTATOR: u8 = 0x08;

  pub fn has(self, flag: u8) -> bool {
    self.0 & flag != 0
  }

  pub fn is_transcriber(self) -> bool {
    self.has(Self::TRANSCRIBER)
  }
  pub fn is_editor(self) -> bool {
    self.has(Self::EDITOR)
  }
  pub fn is_translator(self) -> bool {
    self.has(Self::TRANSLATOR)
  }
  pub fn is_annotator(self) -> bool {
    self.has(Self::ANNOTATOR)
  }
}

/// A documentation contributor — the human who writes and maintains AURA files.
///
/// Distinct from `PersonNode` (content contributors). Defined in
/// `info/annotators.aura`. Referenced via `@annotator/id`.
///
/// An annotator contributing to multiple catalogs has one global record
/// at `@aduki.org/annotators/{id}`, shared across all catalogs.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnnotatorNode {
  /// Generated ID, e.g. `"p9xb3mn"`. Same `p` prefix as `PersonNode`.
  pub id: [u8; 8],
  /// Display name → string pool reference.
  pub name: StringRef,
  /// Role bitfield — see `AnnotatorRoles` constants.
  pub roles: AnnotatorRoles,
  /// ISO 3166-1 alpha-2 country code.
  pub country: [u8; 2],
  /// Contact URI (email or other) — optional.
  pub contact: Option<StringRef>,
}
