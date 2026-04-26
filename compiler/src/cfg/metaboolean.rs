//! Metaboolean vocabulary loader — reads `meta/metaboolean.aura`.
//!
//! Maps custom boolean literals to binary values (0 or 1) so the
//! compiler can recognize domain-specific terms without touching the
//! core engine. The map is built once at compile time and consulted by
//! the linter and emitter for bare values on boolean-typed fields.
//!
//! # File format
//!
//! ```aura
//! ## FILE: meta/metaboolean.aura
//!
//! booleans::
//!
//!   cleared::
//!     true-maps-to  -> live
//!     false-maps-to -> dark
//!
//!   published::
//!     true-maps-to  -> live
//!     false-maps-to -> dark
//!
//!   active::
//!     true-maps-to  -> live
//!     false-maps-to -> dark
//! ```
//!
//! Each inner block names a custom literal. `true-maps-to -> live` means
//! when the compiler sees `cleared -> live` it emits integer 1; the key
//! `cleared` is recognized as a boolean field and W004 linting is
//! suppressed for it.
//!
//! Falls back to an empty map (standard `live`/`dark` only) if the file
//! does not exist.

use std::collections::HashMap;
use std::path::Path;

/// A mapping from custom domain literals to their binary values.
///
/// `resolve("cleared")` → `Some(1)` when `cleared` maps to `live`.
/// `resolve("blocked")` → `Some(0)` when `blocked` maps to `dark`.
#[derive(Debug, Clone)]
pub struct BooleanMap {
  inner: HashMap<String, u8>,
}

impl BooleanMap {
  /// Returns an empty map (only the built-in `live`/`dark` apply).
  pub fn empty() -> Self {
    Self {
      inner: HashMap::new(),
    }
  }

  /// Returns the binary value (0 or 1) for a custom literal, if declared.
  pub fn resolve(&self, literal: &str) -> Option<u8> {
    self.inner.get(literal).copied()
  }

  /// Returns `true` if this literal is declared as a boolean.
  pub fn contains(&self, literal: &str) -> bool {
    self.inner.contains_key(literal)
  }

  /// Returns an iterator over all declared boolean key names.
  pub fn keys(&self) -> impl Iterator<Item = &str> {
    self.inner.keys().map(|s| s.as_str())
  }
}

/// Loads `meta/metaboolean.aura` from `project`.
///
/// Returns [`BooleanMap::empty`] if the file does not exist or cannot
/// be read — compilation continues normally with built-in booleans.
pub fn load(project: &Path) -> BooleanMap {
  let path = project.join("meta").join("metaboolean.aura");
  if !path.exists() {
    return BooleanMap::empty();
  }
  match std::fs::read_to_string(&path) {
    Ok(text) => parse(&text),
    Err(_) => BooleanMap::empty(),
  }
}

// -------------------------------------------------------------------- //
// Parser

fn parse(text: &str) -> BooleanMap {
  let mut map: HashMap<String, u8> = HashMap::new();

  // Parser state.
  let mut current_key: Option<String> = None;
  let mut true_val: Option<u8> = None;
  let mut false_val: Option<u8> = None;

  for line in text.lines() {
    let trimmed = line.trim();

    // Skip comments, dividers, and blank lines.
    if trimmed.is_empty() || trimmed.starts_with("##") || trimmed.starts_with("--") {
      continue;
    }

    // Top-level `booleans::` block — resets state.
    if trimmed == "booleans::" {
      flush(&mut map, &mut current_key, &mut true_val, &mut false_val);
      continue;
    }

    // Inner keyword block: `cleared::`, `published::`, etc.
    if let Some(kw) = trimmed.strip_suffix("::") {
      let kw = kw.trim();
      // Ignore the outer `booleans` keyword if encountered again.
      if kw == "booleans" {
        continue;
      }
      flush(&mut map, &mut current_key, &mut true_val, &mut false_val);
      current_key = Some(kw.to_string());
      continue;
    }

    // Field: `true-maps-to -> live` / `false-maps-to -> dark`
    if let Some(arrow) = trimmed.find("->") {
      let key = trimmed[..arrow].trim();
      let val = trimmed[arrow + 2..].trim().trim_matches('"');
      let bit: Option<u8> = match val {
        "live" | "true" | "1" => Some(1),
        "dark" | "false" | "0" => Some(0),
        _ => None,
      };
      if let (Some(b), Some(_)) = (bit, current_key.as_ref()) {
        match key {
          "true-maps-to" => true_val = Some(b),
          "false-maps-to" => false_val = Some(b),
          _ => {}
        }
      }
    }
  }

  // Flush the final block.
  flush(&mut map, &mut current_key, &mut true_val, &mut false_val);

  BooleanMap { inner: map }
}

/// Commits the current keyword block into the map.
///
/// The "true" form (e.g. `cleared`) is inserted directly.
/// The "false" form is inserted with an internal `!` prefix
/// (e.g. `!cleared`) — used by the linter to identify false-side literals.
fn flush(
  map: &mut HashMap<String, u8>,
  current_key: &mut Option<String>,
  true_val: &mut Option<u8>,
  false_val: &mut Option<u8>,
) {
  if let Some(key) = current_key.take() {
    if let Some(v) = true_val.take() {
      map.insert(key.clone(), v);
    }
    if let Some(v) = false_val.take() {
      // False-side uses `!key` prefix so it doesn't shadow the true form.
      map.insert(format!("!{}", key), v);
    }
  }
  *true_val = None;
  *false_val = None;
}
