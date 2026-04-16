//! Ignore list reader — reads `configs/ignore.aura`.
//!
//! Files and paths listed in `configs/ignore.aura` are excluded from
//! both compilation and `.history/` tracking.
//!
//! # Built-in exclusions (always applied regardless of ignore.aura)
//!
//! | Path          | Reason                                           |
//! | ------------- | ------------------------------------------------ |
//! | `configs/`    | Toolchain config — never compiled, never tracked |
//! | `.history/`   | History store — read by CLI, not compiled        |
//! | `artwork/`    | Binary image assets — not compiled               |
//! | `motion/`     | Binary video assets — not compiled               |
//! | `trailers/`   | Binary video assets — not compiled               |
//! | `stems/`      | Audio stems — not compiled                       |
//! | `dist/`       | Compiler output — never re-compiled              |

use crate::error::{CompileError, Result};
use std::path::Path;

/// Built-in paths that are always excluded from compilation and tracking.
pub const BUILTIN: &[&str] = &[
  "configs", ".history", "artwork", "motion", "trailers", "stems", "dist",
];

/// The effective exclusion list for a project.
pub struct IgnoreList {
  /// Paths excluded from compilation (relative to project root).
  patterns: Vec<String>,
}

impl IgnoreList {
  /// Creates an ignore list containing only the built-in exclusions.
  pub fn builtin() -> Self {
    Self {
      patterns: BUILTIN.iter().map(|s| s.to_string()).collect(),
    }
  }

  /// Loads `configs/ignore.aura` and merges it with the built-in list.
  pub fn load(project_root: impl AsRef<Path>) -> Result<Self> {
    let path = project_root.as_ref().join("configs").join("ignore.aura");
    let mut list = Self::builtin();

    if path.exists() {
      let text = std::fs::read_to_string(&path)
        .map_err(|e| CompileError::msg(format!("cannot read configs/ignore.aura: {}", e)))?;
      list.parse_ignore(&text);
    }

    Ok(list)
  }

  /// Returns `true` if the given path should be excluded.
  ///
  /// `path` is a relative path from the project root.
  pub fn is_excluded(&self, path: impl AsRef<Path>) -> bool {
    let path_str = path.as_ref().to_string_lossy();
    for pattern in &self.patterns {
      if path_str.starts_with(pattern.as_str()) || path_str == pattern.as_str() {
        return true;
      }
    }
    false
  }

  fn parse_ignore(&mut self, text: &str) {
    for line in text.lines() {
      let trimmed = line.trim();
      // Skip comments, empty lines, and namespace headers.
      if trimmed.is_empty()
        || trimmed.starts_with("##")
        || trimmed.starts_with("--")
        || trimmed.ends_with("::")
      {
        continue;
      }
      // List entries: `- path/to/exclude`
      if let Some(rest) = trimmed.strip_prefix("- ") {
        let p = rest.trim().trim_matches('"').to_string();
        if !p.is_empty() && !self.patterns.contains(&p) {
          self.patterns.push(p);
        }
      }
    }
  }
}
