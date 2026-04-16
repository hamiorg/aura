//! Known standard AURA keys for the W006 unknown-key lint rule.
//!
//! This set is derived from `compiler/keywords.md`. Keys not in this
//! set trigger a W006 warning in strict mode (`--strict` or
//! `directives::strict -> live`).

use std::collections::HashSet;

/// Returns the set of all standard AURA field keys.
pub fn valid_keys() -> HashSet<&'static str> {
  let mut s = HashSet::new();

  // Schema / directives
  s.extend([
    "root",
    "kind",
    "namespace",
    "lang",
    "annotator",
    "annotators",
    "strict",
    "mood-vocabulary",
    "store",
    "variation-default",
  ]);

  // Manifest / collection
  s.extend([
    "name",
    "creator",
    "author",
    "authors",
    "created",
    "updated",
    "version",
    "released",
    "cover",
    "access",
    "embargo",
    "explicit",
    "cleared",
    "published",
    "featured",
    "live",
    "dark",
    "authored",
    "revised",
  ]);

  // People / annotators
  s.extend([
    "first", "middle", "last", "screen", "legal", "contact", "roles", "country",
  ]);

  // Content nodes
  s.extend(["time", "text", "lyrics", "note", "lang"]);

  // Vocabulary
  s.extend([
    "genre", "genres", "role", "roles", "mood", "moods", "slug", "parent", "region",
  ]);

  // Media / assets
  s.extend([
    "url", "ratio", "loop", "duration", "format", "codec", "rating",
  ]);

  // Industry entities
  s.extend(["logo", "website", "founded"]);

  // Availability / distribution
  s.extend([
    "platform", "price", "watch", "buy", "rent", "download", "blocked", "holder", "scope",
  ]);

  // History
  s.extend([
    "source",
    "used-at",
    "element",
    "writers",
    "confidence",
    "marks",
    "aura",
    "atom",
    "hami",
    "atlas",
  ]);

  // Support nodes
  s.extend([
    "trigger",
    "signal",
    "target",
    "condition",
    "via",
    "max",
    "performer",
  ]);

  s
}
