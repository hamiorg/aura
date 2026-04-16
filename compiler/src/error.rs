//! Compiler error types.
//!
//! All pipeline stages return `Result<_, CompileError>`. Errors carry a
//! `Span` locating the problem in the source file so the CLI can emit
//! annotated diagnostics pointing to the exact line and column.

use std::fmt;
use std::path::PathBuf;

/// Severity level of a compiler diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  /// Informational note — does not prevent output.
  Note,
  /// Style or convention warning — does not prevent output.
  Warning,
  /// Hard error — compilation aborts before emitting output.
  Error,
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Note => f.write_str("note"),
      Self::Warning => f.write_str("warning"),
      Self::Error => f.write_str("error"),
    }
  }
}

/// A byte-range location inside a source file.
///
/// All offsets are 0-indexed bytes into the UTF-8 source buffer.
/// Line and column are 1-indexed for display purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
  /// Byte offset of the first character of the relevant token.
  pub start: usize,
  /// Byte offset one past the last character.
  pub end: usize,
  /// 1-indexed line number.
  pub line: u32,
  /// 1-indexed column number (byte column, not Unicode column).
  pub col: u32,
}

impl Span {
  pub fn new(start: usize, end: usize, line: u32, col: u32) -> Self {
    Self {
      start,
      end,
      line,
      col,
    }
  }

  /// A span covering a single byte position.
  pub fn point(offset: usize, line: u32, col: u32) -> Self {
    Self {
      start: offset,
      end: offset + 1,
      line,
      col,
    }
  }
}

impl fmt::Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.line, self.col)
  }
}

/// A compiler diagnostic — a located, levelled message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
  pub level: Level,
  pub message: String,
  pub file: Option<PathBuf>,
  pub span: Option<Span>,
  /// Optional suggestion shown below the error.
  pub hint: Option<String>,
}

impl Diagnostic {
  pub fn error(message: impl Into<String>) -> Self {
    Self {
      level: Level::Error,
      message: message.into(),
      file: None,
      span: None,
      hint: None,
    }
  }

  pub fn warning(message: impl Into<String>) -> Self {
    Self {
      level: Level::Warning,
      message: message.into(),
      file: None,
      span: None,
      hint: None,
    }
  }

  pub fn note(message: impl Into<String>) -> Self {
    Self {
      level: Level::Note,
      message: message.into(),
      file: None,
      span: None,
      hint: None,
    }
  }

  pub fn with_file(mut self, path: PathBuf) -> Self {
    self.file = Some(path);
    self
  }

  pub fn with_span(mut self, span: Span) -> Self {
    self.span = Some(span);
    self
  }

  pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
    self.hint = Some(hint.into());
    self
  }
}

impl fmt::Display for Diagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(path) = &self.file {
      write!(f, "{}:", path.display())?;
    }
    if let Some(span) = &self.span {
      write!(f, "{}: ", span)?;
    }
    write!(f, "{}: {}", self.level, self.message)?;
    if let Some(hint) = &self.hint {
      write!(f, "\n  hint: {}", hint)?;
    }
    Ok(())
  }
}

/// The primary error type returned by all compiler pipeline stages.
#[derive(Debug)]
pub struct CompileError {
  /// One or more diagnostics that describe what went wrong.
  pub diagnostics: Vec<Diagnostic>,
}

impl CompileError {
  /// Creates an error with a single diagnostic.
  pub fn single(diag: Diagnostic) -> Self {
    Self {
      diagnostics: vec![diag],
    }
  }

  /// Creates an error from a bare message string.
  pub fn msg(message: impl Into<String>) -> Self {
    Self::single(Diagnostic::error(message))
  }

  /// Creates an error with file + span context.
  pub fn at(file: PathBuf, span: Span, message: impl Into<String>) -> Self {
    Self::single(Diagnostic::error(message).with_file(file).with_span(span))
  }

  /// Returns `true` if any diagnostic is at `Error` level.
  pub fn is_fatal(&self) -> bool {
    self.diagnostics.iter().any(|d| d.level == Level::Error)
  }

  /// Merges another error's diagnostics into this one.
  pub fn merge(&mut self, other: CompileError) {
    self.diagnostics.extend(other.diagnostics);
  }
}

impl fmt::Display for CompileError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, diag) in self.diagnostics.iter().enumerate() {
      if i > 0 {
        writeln!(f)?;
      }
      write!(f, "{}", diag)?;
    }
    Ok(())
  }
}

impl std::error::Error for CompileError {}

/// Shorthand: convert a `CompileError` into a `Vec<Diagnostic>` for
/// accumulated error collection across pipeline stages.
pub type Result<T> = std::result::Result<T, CompileError>;
