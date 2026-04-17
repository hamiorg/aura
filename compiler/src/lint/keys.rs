//! Known standard AURA keys for the W006 unknown-key lint rule.
//!
//! This set is derived from `compiler/keywords.md`. Keys not in this
//! set trigger a W006 warning in strict mode (`--strict` or
//! `directives::strict -> live`).
//!
//! # Plural / singular convention
//!
//! AURA enforces a strict plural/singular rule for reference keys:
//!
//! - **Singular key** (`@domain/id`)      → single-ID reference
//!   e.g. `producer -> @person/p4xt9k2`
//! - **Plural key**  (`@domain/[id,…]`)  → multi-ID reference
//!   e.g. `producers -> @people/[p9gregk, p8paule]`
//!
//! Both forms are in the standard vocabulary so neither triggers W006.

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
    "annotators",   // plural: multi-annotator file
    "strict",
    "mood-vocabulary",
    "store",
    "variation-default",
    // name.aura entry block fields
    "id",
    "slug",
    "folder",
  ]);

  // Manifest / collection
  s.extend([
    "name",
    "creator",
    "author",
    "authors",      // plural: multiple authors
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

  // People / person metadata
  s.extend([
    "first", "middle", "last", "screen", "legal", "contact",
    "language", "country", "born", "bio", "city",
  ]);

  // Content nodes
  s.extend(["time", "text", "lyrics", "note", "lang", "duration"]);

  // Vocabulary (singular AND plural — both are standard)
  s.extend([
    "genre",   "genres",   // singular: @genre/slug | plural: @genres/[id1,id2]
    "role",    "roles",    // singular: @role/slug  | plural: @roles/[id1,id2]
    "mood",    "moods",    // singular: @mood/slug  | plural: @moods/[id1,id2]
    "slug", "parent", "region",
    // Mood-specific vocabulary fields
    "valence", "energy",
  ]);

  // Music / media metadata (singular and plural forms)
  s.extend([
    "bpm", "grid", "key", "isrc", "iswc", "license", "expires",
    "show",
    "season",    "seasons",    // singular/plural
    "episode",   "episodes",   // singular/plural
    "track",     "tracks",     // singular/plural
    "scene",     "scenes",     // singular/plural
    "act",       "acts",       // singular/plural
    "chapter",   "chapters",   // singular/plural
    "segment",   "segments",   // singular/plural
    "section",   "sections",   // singular/plural
    "variant",   "variants",   // singular/plural
    "synopsis",  "tags",  "links",
    "family",    "active",  "stem",
    "label",     "labels",     // singular/plural label refs
    "locale",    "script",     "territory",
    "count",     "index",      "hash",
    "main",      "vocals",
    "producer",  "producers",  // singular/plural
    "writer",    "writers",    // singular/plural
    "mixer",     "master",
    "director",  "directors",  // singular/plural
    "editor",    "editors",    // singular/plural
    "narrator",  "narrators",  // singular/plural
    "cast",
    "host",      "hosts",      // singular/plural
    "guest",     "guests",     // singular/plural
    "speaker",   "speakers",   // singular/plural
    "performer", "performers", // singular/plural
    "instrument","instruments",// singular/plural
    "sample",    "samples",    // singular/plural
  ]);

  // Media / assets (art, motion, industry) — singular and plural
  s.extend([
    "art",     "arts",     // singular/plural art refs
    "motion",  "motions",  // singular/plural motion refs
    "trailer", "trailers", // singular/plural trailer refs
    "studio",  "studios",  // singular/plural studio refs
    "url", "ratio", "loop", "format", "codec", "rating", "logo", "website",
  ]);

  // Industry entities
  s.extend(["founded"]);

  // Availability / distribution
  s.extend([
    "platform", "price", "currency", "window", "drm", "quality",
    "watch", "buy", "rent", "download", "blocked", "holder", "scope",
  ]);

  // LLM / toolchain configs
  s.extend(["provider", "model", "endpoint", "auth", "env"]);

  // History / provenance
  s.extend([
    "source", "used-at", "element", "confidence", "marks",
    "aura", "atom", "hami", "atlas",
  ]);

  // Support nodes
  s.extend([
    "trigger", "signal", "target", "condition", "via", "max",
    // ref: cross-boundary reference within a collection member block
    "ref",
    // persons-ref: people index reference
    "persons-ref",
  ]);

  // Approved hyphenated keys
  s.extend([
    "pre-chorus", "post-chorus",
    "lead-vocal", "co-writer", "voice-over",
    "rights-holder", "fill-policy",
  ]);

  s
}
