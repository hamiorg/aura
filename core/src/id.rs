//! ID generation and prefix registry.
//!
//! Every AURA ID has the format `{prefix}{6 alphanumeric chars}` using
//! charset `a-z0-9` (36 possible values per position → 36^6 = 2,176,782,336
//! unique values per prefix). IDs are never hand-authored.

/// All valid type prefixes for AURA IDs.
///
/// A prefix encodes what the object is. Any system reading an ID can
/// determine the object class from the prefix alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prefix {
  /// `t` — audio music track
  Track,
  /// `c` — album, EP, single, or compilation manifest
  Collection,
  /// `p` — contributor, creator, or any named individual
  Person,
  /// `v` — alternate version of any content file
  Variant,
  /// `ep` — single episode in a series or podcast
  Episode,
  /// `sn` — season within a series or podcast
  Season,
  /// `s` — season file within a series folder
  SeasonItem,
  /// `tv` — TV, podcast, or episodic series root manifest
  Series,
  /// `f` — feature or short film
  Film,
  /// `dc` — documentary work
  Documentary,
  /// `pc` — podcast series root manifest
  Podcast,
  /// `an` — animated or anime series root manifest
  Animation,
  /// `sp` — speech, lecture, talk, or commencement address
  Speech,
  /// `b` — audiobook
  Book,
  /// `mv` — music video
  MusicVideo,
  /// `sg` — single release
  Single,
  /// `cy` — discrete interview file
  Interview,
  /// `r` — rights or licensing declaration file
  Rights,
  /// `i` — info document (metadata, credits, labels)
  Info,
  /// `tx` — history take (immutable version snapshot)
  Take,
  /// `st` — production studio or broadcast network entity
  Studio,
  /// `lb` — record label or publishing imprint
  Label,
  /// `ar` — static image art asset (cover art, poster)
  Art,
  /// `mo` — animated motion cover or looping video asset
  Motion,
  /// `tr` — promotional trailer or preview clip
  Trailer,
}

impl Prefix {
  /// Returns the string prefix for this variant.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Track => "t",
      Self::Collection => "c",
      Self::Person => "p",
      Self::Variant => "v",
      Self::Episode => "ep",
      Self::Season => "sn",
      Self::SeasonItem => "s",
      Self::Series => "tv",
      Self::Film => "f",
      Self::Documentary => "dc",
      Self::Podcast => "pc",
      Self::Animation => "an",
      Self::Speech => "sp",
      Self::Book => "b",
      Self::MusicVideo => "mv",
      Self::Single => "sg",
      Self::Interview => "cy",
      Self::Rights => "r",
      Self::Info => "i",
      Self::Take => "tx",
      Self::Studio => "st",
      Self::Label => "lb",
      Self::Art => "ar",
      Self::Motion => "mo",
      Self::Trailer => "tr",
    }
  }

  /// Returns the `Prefix` that matches a given string prefix, if any.
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "t" => Some(Self::Track),
      "c" => Some(Self::Collection),
      "p" => Some(Self::Person),
      "v" => Some(Self::Variant),
      "ep" => Some(Self::Episode),
      "sn" => Some(Self::Season),
      "s" => Some(Self::SeasonItem),
      "tv" => Some(Self::Series),
      "f" => Some(Self::Film),
      "dc" => Some(Self::Documentary),
      "pc" => Some(Self::Podcast),
      "an" => Some(Self::Animation),
      "sp" => Some(Self::Speech),
      "b" => Some(Self::Book),
      "mv" => Some(Self::MusicVideo),
      "sg" => Some(Self::Single),
      "cy" => Some(Self::Interview),
      "r" => Some(Self::Rights),
      "i" => Some(Self::Info),
      "tx" => Some(Self::Take),
      "st" => Some(Self::Studio),
      "lb" => Some(Self::Label),
      "ar" => Some(Self::Art),
      "mo" => Some(Self::Motion),
      "tr" => Some(Self::Trailer),
      _ => None,
    }
  }
}

/// Charset used for the 6-character body of every AURA ID.
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

/// A generated AURA ID.
///
/// Format: `{prefix}{body}` where body is exactly 6 characters from
/// the charset `a-z0-9`. Maximum total length is 8 characters (2-char
/// prefix + 6-char body).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuraId {
  /// The raw string, e.g. `"t7xab3c"`.
  inner: String,
}

impl AuraId {
  /// Creates an `AuraId` from a raw string without validation.
  /// Prefer `IdGen::generate` to produce validated IDs.
  pub fn from_raw(s: impl Into<String>) -> Self {
    Self { inner: s.into() }
  }

  /// Returns the string representation of this ID.
  pub fn as_str(&self) -> &str {
    &self.inner
  }

  /// Parses the prefix from this ID, if it matches a known prefix.
  pub fn prefix(&self) -> Option<Prefix> {
    // Try two-char prefix first, then one-char.
    if self.inner.len() >= 2 {
      if let Some(p) = Prefix::from_str(&self.inner[..2]) {
        return Some(p);
      }
    }
    if !self.inner.is_empty() {
      Prefix::from_str(&self.inner[..1])
    } else {
      None
    }
  }

  /// Returns the 6-character body (the part after the prefix).
  pub fn body(&self) -> &str {
    let prefix_len = self.prefix().map(|p| p.as_str().len()).unwrap_or(0);
    &self.inner[prefix_len..]
  }

  /// Validates that the ID body is exactly 6 lowercase alphanumeric chars.
  pub fn is_valid(&self) -> bool {
    let body = self.body();
    body.len() == 6 && body.bytes().all(|b| CHARSET.contains(&b))
  }
}

impl std::fmt::Display for AuraId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.inner)
  }
}

/// ID generator for AURA objects.
///
/// Generates IDs on demand, checking each against the active registry
/// before returning. IDs are never hand-authored.
///
/// In local development the registry is a flat file at the project root.
/// In cloud deployments the store's ID registry is authoritative.
pub struct IdGen {
  registry: std::collections::HashSet<String>,
}

impl IdGen {
  /// Creates a new empty generator (no registered IDs).
  pub fn new() -> Self {
    Self {
      registry: std::collections::HashSet::new(),
    }
  }

  /// Creates a generator pre-seeded with an existing set of IDs.
  pub fn with_registry(ids: impl IntoIterator<Item = String>) -> Self {
    Self {
      registry: ids.into_iter().collect(),
    }
  }

  /// Generates a unique ID for the given prefix.
  ///
  /// Retries on collision. Returns `None` only if the PRNG is
  /// unavailable (should not happen in practice).
  pub fn generate(&mut self, prefix: Prefix) -> AuraId {
    loop {
      let body = self.random_body();
      let id = format!("{}{}", prefix.as_str(), body);
      if !self.registry.contains(&id) {
        self.registry.insert(id.clone());
        return AuraId::from_raw(id);
      }
    }
  }

  /// Registers an externally issued ID so future generations avoid it.
  pub fn register(&mut self, id: &AuraId) {
    self.registry.insert(id.inner.clone());
  }

  fn random_body(&self) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    // Simple non-crypto RNG seeded by time + registry size.
    // In production, replace with a cryptographic RNG.
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .subsec_nanos() as usize
      ^ (self.registry.len().wrapping_mul(0x9e3779b9));

    let mut s = String::with_capacity(6);
    let mut v = seed;
    for _ in 0..6 {
      v = v
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
      s.push(CHARSET[v % CHARSET.len()] as char);
    }
    s
  }
}

impl Default for IdGen {
  fn default() -> Self {
    Self::new()
  }
}
