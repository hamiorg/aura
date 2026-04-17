//! AURA compiler logger.
//!
//! A lightweight, zero-dependency structured logger for the AURA toolchain.
//!
//! # Usage
//!
//! ```rust,ignore
//! use compiler::logs::Logger;
//!
//! let log = Logger::new();
//!
//! log.compile("Building project…");
//! log.parse("Parsing tracks/t7xab3c.aura");
//! log.lint("Running W003 — interval nodes missing `time`");
//! log.warn("moods.aura", 1, Some("W003"), "node `moods` has no `time` field", None);
//! log.error("credits.aura", 5, None, "expected `->`, got Comma", Some("Add `->` after the key name"));
//! log.success("Compilation complete — 12 files compiled");
//! ```
//!
//! All output goes to **stderr** so it does not pollute stdout (which may
//! carry machine-readable output in future piped workflows).

use crate::logs::formatter;

// -------------------------------------------------------------------- //

/// The AURA compiler logger.
///
/// Create once and pass by reference; it holds no mutable state.
/// Thread-safe: all methods take `&self` and use `eprintln!`.
pub struct Logger;

impl Logger {
  /// Create a new logger instance.
  pub fn new() -> Self {
    Self
  }

  // ------------------------------------------------------------------ //
  // Phase loggers — no file/line context

  /// Log a top-level compiler phase message (e.g. `"Building project…"`).
  pub fn compile(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("compile", msg, None));
  }

  /// Log a lexer phase message.
  pub fn lex(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("lex", msg, None));
  }

  /// Log a parser phase message.
  pub fn parse(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("parse", msg, None));
  }

  /// Log a linter phase message.
  pub fn lint(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("lint", msg, None));
  }

  /// Log an emitter phase message.
  pub fn emit(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("emit", msg, None));
  }

  // ------------------------------------------------------------------ //
  // General-purpose loggers

  /// General informational message.
  pub fn info(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("info", msg, None));
  }

  /// Debug message (only useful with verbose output).
  pub fn debug(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("debug", msg, None));
  }

  /// Compiler note — supplementary information attached to a diagnostic.
  pub fn note(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("note", msg, None));
  }

  /// Success message — compilation or operation completed cleanly.
  pub fn success(&self, msg: &str) {
    eprintln!("{}", formatter::format_message("success", msg, None));
  }

  // ------------------------------------------------------------------ //
  // Diagnostic loggers — carry file, line, optional code, and hint

  /// Emit a warning diagnostic.
  ///
  /// # Arguments
  /// * `file`  — source file name (e.g. `"credits.aura"`)
  /// * `line`  — 1-indexed source line
  /// * `code`  — optional rule code, e.g. `Some("W003")`
  /// * `msg`   — human-readable message
  /// * `hint`  — optional actionable fix hint
  pub fn warn(&self, file: &str, line: u32, code: Option<&str>, msg: &str, hint: Option<&str>) {
    eprintln!("{}", formatter::format_diag("warn", file, line, code, msg, hint));
  }

  /// Emit an error diagnostic.
  ///
  /// # Arguments
  /// * `file`  — source file name (e.g. `"credits.aura"`)
  /// * `line`  — 1-indexed source line
  /// * `code`  — optional rule code, e.g. `Some("E001")`
  /// * `msg`   — human-readable message
  /// * `hint`  — optional actionable fix hint
  pub fn error(&self, file: &str, line: u32, code: Option<&str>, msg: &str, hint: Option<&str>) {
    eprintln!("{}", formatter::format_diag("error", file, line, code, msg, hint));
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}
