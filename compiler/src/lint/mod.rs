//! Lint module — static analysis of AURA source documents.
//!
//! # Usage
//!
//! ```rust,ignore
//! use compiler::lint::Linter;
//!
//! let doc    = Parser::new(src).parse()?;
//! let result = Linter::new(false).lint(&doc, &file_path);
//! result.print();
//! if result.has_errors() { return Err(...); }
//! ```
//!
//! # Rule codes
//!
//! | Code | Level   | Description                                           |
//! | ---- | ------- | ----------------------------------------------------- |
//! | W001 | Warning | Boolean `true`/`false` — use `live`/`dark`            |
//! | W002 | Warning | Deprecated key (`thumbnail`, `artwork`)               |
//! | W003 | Warning | Interval-indexed node missing `time` field            |
//! | W004 | Warning | `manifest::` missing `name`                           |
//! | W005 | Warning | `manifest::` missing `creator`/`author`               |
//! | W006 | Warning | Key not in standard vocabulary (strict mode only)     |
//! | E001 | Error   | Required field (`!`) absent                           |

pub mod keys;
pub mod rules;

use crate::error::Level;
use crate::parse::ast::Document;
use std::path::Path;

/// A single lint diagnostic.
#[derive(Debug, Clone)]
pub struct LintDiag {
  pub code: &'static str,
  pub level: Level,
  pub msg: String,
  pub file: std::path::PathBuf,
  pub line: u32,
}

/// The accumulated result of linting one document.
#[derive(Debug, Default)]
pub struct LintResult {
  pub diags: Vec<LintDiag>,
}

impl LintResult {
  /// Returns `true` if any diagnostic is at `Error` level.
  pub fn has_errors(&self) -> bool {
    self.diags.iter().any(|d| d.level == Level::Error)
  }

  /// Prints all diagnostics to stderr.
  pub fn print(&self) {
    for d in &self.diags {
      let lvl = match d.level {
        Level::Error => "error",
        Level::Warning => "warning",
        Level::Note => "note",
      };
      eprintln!(
        "{}:{}: {}: [{}] {}",
        d.file.display(),
        d.line,
        lvl,
        d.code,
        d.msg,
      );
    }
  }
}

/// The linter — runs all rules against a parsed `Document`.
pub struct Linter {
  /// When `true`, W006 (unknown key) is also emitted.
  strict: bool,
}

impl Linter {
  pub fn new(strict: bool) -> Self {
    Self { strict }
  }

  /// Run all applicable rules and return a `LintResult`.
  pub fn lint(&self, doc: &Document<'_>, file: &Path) -> LintResult {
    let mut result = LintResult::default();

    macro_rules! run {
      ($rule:expr) => {
        result.diags.extend($rule(doc, file));
      };
    }

    run!(rules::w001);
    run!(rules::w002);
    run!(rules::w003);
    run!(rules::w004);
    run!(rules::w005);
    run!(rules::e001);

    if self.strict {
      run!(rules::w006);
    }

    result
  }
}
