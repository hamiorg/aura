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
    // name.aura entry block fields
    "id",
    "slug",
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
    "first", "middle", "last", "screen", "legal", "contact", "roles", "country", "born", "bio",
    "city",
  ]);

  // Content nodes
  s.extend(["time", "text", "lyrics", "note", "lang", "duration"]);

  // Vocabulary
  s.extend([
    "genre", "genres", "role", "roles", "mood", "moods", "slug", "parent", "region",
    // Mood-specific vocabulary fields
    "valence", "energy",
  ]);

  // Music / media metadata
  s.extend([
    "bpm",
    "grid",
    "key",
    "isrc",
    "iswc",
    "license",
    "expires",
    "show",
    "season",
    "episode",
    "synopsis",
    "tags",
    "links",
    "family",
    "active",
    "stem",
    "label",
    "locale",
    "script",
    "territory",
    "count",
    "index",
    "hash",
    "main",
    "vocals",
    "producer",
    "writer",
    "mixer",
    "master",
    "director",
    "editor",
    "narrator",
    "cast",
    "host",
    "guest",
    "speaker",
    "speakers",
  ]);

  // Media / assets
  s.extend([
    "url", "ratio", "loop", "format", "codec", "rating", "motion", "trailer", "studio", "logo",
    "website",
  ]);

  // Industry entities
  s.extend(["founded"]);

  // Availability / distribution
  s.extend([
    "platform", "price", "currency", "window", "drm", "quality", "watch", "buy", "rent",
    "download", "blocked", "holder", "scope",
  ]);

  // LLM / toolchain configs
  s.extend(["provider", "model", "endpoint", "auth", "env"]);

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
    // ref: cross-boundary reference within a collection member block
    "ref",
    // persons-ref: people index reference
    "persons-ref",
  ]);

  // Approved hyphenated keys
  s.extend([
    "pre-chorus",
    "post-chorus",
    "lead-vocal",
    "co-writer",
    "voice-over",
    "rights-holder",
    "fill-policy",
  ]);

  s
}
