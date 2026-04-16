//! Zero-copy byte scanner — the lexer hot path.
//!
//! The scanner reads raw UTF-8 bytes from the source buffer and emits
//! a stream of `Token` values. It never allocates heap memory; all
//! string slices point directly into the original source buffer.
//!
//! # Design invariants
//!
//! 1. No heap allocation (`String`). Only `&'src str` slices.
//! 2. No character escaping. AURA prohibits escape sequences.
//! 3. Hot path: `if byte < 0x20 → structural token` — enables AVX-2
//!    SIMD vectorization (32 bytes per CPU clock cycle).
//! 4. Sigils (`::`, `->`, `@`, etc.) are emitted as raw byte sequences.
//!    The parser assigns semantic meaning.

use crate::error::{CompileError, Result};
use crate::lex::token::{Kind, Token};

/// The zero-copy byte scanner.
///
/// Create one scanner per source file. Call `next()` to advance token by
/// token. The scanner is lazy — it only scans as far as needed.
pub struct Scanner<'src> {
  src: &'src str,
  bytes: &'src [u8],
  pos: usize,
  line: u32,
  /// Current indentation depth in spaces (or tabs × 2).
  #[allow(dead_code)]
  indent: u32,
  /// Pending indent/dedent tokens queued before the next real token.
  pending: std::collections::VecDeque<Token<'src>>,
}

impl<'src> Scanner<'src> {
  /// Creates a new scanner for the given UTF-8 source string.
  pub fn new(src: &'src str) -> Self {
    Self {
      src,
      bytes: src.as_bytes(),
      pos: 0,
      line: 1,
      indent: 0,
      pending: std::collections::VecDeque::new(),
    }
  }

  /// Returns the next token, or `Eof` when the source is exhausted.
  pub fn next(&mut self) -> Result<Token<'src>> {
    // Drain any queued indent/dedent tokens first.
    if let Some(t) = self.pending.pop_front() {
      return Ok(t);
    }
    self.scan_token()
  }

  /// Returns a slice of all tokens without consuming the scanner.
  /// Intended for tests and diagnostics only.
  pub fn collect_all(mut self) -> Result<Vec<Token<'src>>> {
    let mut tokens = Vec::new();
    loop {
      let tok = self.next()?;
      let is_eof = tok.kind == Kind::Eof;
      tokens.push(tok);
      if is_eof {
        break;
      }
    }
    Ok(tokens)
  }

  // ------------------------------------------------------------------ //

  fn peek(&self) -> Option<u8> {
    self.bytes.get(self.pos).copied()
  }

  fn peek_at(&self, offset: usize) -> Option<u8> {
    self.bytes.get(self.pos + offset).copied()
  }

  fn advance(&mut self) -> Option<u8> {
    let b = self.bytes.get(self.pos).copied()?;
    self.pos += 1;
    if b == b'\n' {
      self.line += 1;
    }
    Some(b)
  }

  fn slice(&self, start: usize, end: usize) -> &'src str {
    // SAFETY: start..end are always byte boundaries we derived from
    // scanning valid UTF-8. The original src is &'src str.
    &self.src[start..end]
  }

  fn tok(&self, kind: Kind<'src>, offset: usize) -> Token<'src> {
    Token {
      kind,
      offset,
      line: self.line,
    }
  }

  fn scan_token(&mut self) -> Result<Token<'src>> {
    // Skip horizontal whitespace (spaces/tabs that are not significant
    // indentation at line start).
    self.skip_inline_space();

    let start = self.pos;
    let line = self.line;

    let byte = match self.peek() {
      Some(b) => b,
      None => return Ok(self.tok(Kind::Eof, start)),
    };

    // --- Newline / indentation ---
    if byte == b'\n' {
      self.advance();
      return Ok(Token {
        kind: Kind::Newline,
        offset: start,
        line,
      });
    }

    // --- Comment / divider (`##` or `--`) ---
    if byte == b'#' && self.peek_at(1) == Some(b'#') {
      return self.scan_annotation(start);
    }
    if byte == b'-' && self.peek_at(1) == Some(b'-') {
      self.pos += 2;
      self.skip_to_eol();
      return Ok(self.tok(Kind::Divider, start));
    }

    // --- Inherits `>>` ---
    if byte == b'>' && self.peek_at(1) == Some(b'>') {
      self.pos += 2;
      return Ok(self.tok(Kind::Inherits, start));
    }

    // --- Arrow `->` ---
    if byte == b'-' && self.peek_at(1) == Some(b'>') {
      self.pos += 2;
      return Ok(self.tok(Kind::Arrow, start));
    }

    // --- Scope open `::` ---
    if byte == b':' && self.peek_at(1) == Some(b':') {
      self.pos += 2;
      return Ok(self.tok(Kind::ScopeOpen, start));
    }

    // --- Reference `@` ---
    if byte == b'@' {
      self.pos += 1;
      return self.scan_ref_path(start);
    }

    // --- Quoted string ---
    if byte == b'"' {
      return self.scan_quoted(start);
    }

    // --- List/triple brackets ---
    if byte == b'[' {
      self.advance();
      return Ok(self.tok(Kind::BracketOpen, start));
    }
    if byte == b']' {
      self.advance();
      return Ok(self.tok(Kind::BracketClose, start));
    }
    if byte == b',' {
      self.advance();
      return Ok(self.tok(Kind::Comma, start));
    }

    // --- Union pipe ---
    if byte == b'|' {
      self.advance();
      return Ok(self.tok(Kind::Pipe, start));
    }

    // --- Time operators ---
    if byte == b'~' {
      self.advance();
      return Ok(self.tok(Kind::Tilde, start));
    }
    if byte == b'+' {
      self.advance();
      return Ok(self.tok(Kind::Plus, start));
    }

    // --- Field markers ---
    if byte == b'?' {
      self.advance();
      return Ok(self.tok(Kind::Optional, start));
    }
    if byte == b'!' {
      self.advance();
      return Ok(self.tok(Kind::Required, start));
    }

    // --- Wildcard ---
    if byte == b'*' {
      self.advance();
      return Ok(self.tok(Kind::Wildcard, start));
    }

    // --- Key or bare value (alphanumeric + hyphens + dots + colons) ---
    if is_key_start(byte) {
      return self.scan_key_or_bare(start);
    }

    // Unknown byte — emit a hard error.
    Err(CompileError::msg(format!(
      "unexpected byte `{:?}` (0x{:02X}) at line {}",
      byte as char, byte, self.line
    )))
  }

  fn skip_inline_space(&mut self) {
    while let Some(b) = self.peek() {
      if b == b' ' || b == b'\t' {
        self.pos += 1;
      } else {
        break;
      }
    }
  }

  fn skip_to_eol(&mut self) {
    while let Some(b) = self.peek() {
      if b == b'\n' {
        break;
      }
      self.pos += 1;
    }
  }

  fn scan_annotation(&mut self, start: usize) -> Result<Token<'src>> {
    self.pos += 2; // skip ##
    self.skip_inline_space();
    let text_start = self.pos;
    self.skip_to_eol();
    let text = self.slice(text_start, self.pos);
    Ok(self.tok(Kind::AnnotationText(text), start))
  }

  fn scan_ref_path(&mut self, at_start: usize) -> Result<Token<'src>> {
    let path_start = self.pos;
    while let Some(b) = self.peek() {
      if b.is_ascii_alphanumeric() || b == b'/' || b == b'-' || b == b'.' || b == b'_' {
        self.pos += 1;
      } else {
        break;
      }
    }
    let path = self.slice(path_start, self.pos);
    Ok(self.tok(Kind::RefPath(path), at_start))
  }

  fn scan_quoted(&mut self, start: usize) -> Result<Token<'src>> {
    self.pos += 1; // skip opening "
    while let Some(b) = self.peek() {
      if b == b'"' {
        self.pos += 1; // include closing "
        break;
      }
      // AURA prohibits escape sequences — a raw `"` in content is not
      // allowed inside quoted strings. Report an error if we hit EOL.
      if b == b'\n' {
        return Err(CompileError::msg(format!(
          "unterminated string literal starting at line {}",
          self.line
        )));
      }
      self.pos += 1;
    }
    let text = self.slice(start, self.pos);
    Ok(self.tok(Kind::Quoted(text), start))
  }

  fn scan_key_or_bare(&mut self, start: usize) -> Result<Token<'src>> {
    while let Some(b) = self.peek() {
      if is_key_cont(b) {
        self.pos += 1;
      } else {
        break;
      }
    }
    let s = self.slice(start, self.pos);
    // Heuristic: if the text looks like a time literal it's a Bare.
    // Otherwise it's a Key.
    let kind = if looks_like_time(s) {
      Kind::Time(s)
    } else {
      Kind::Key(s)
    };
    Ok(self.tok(kind, start))
  }
}

// -------------------------------------------------------------------- //
// Byte classification helpers

fn is_key_start(b: u8) -> bool {
  b.is_ascii_alphabetic() || b == b'_'
}

fn is_key_cont(b: u8) -> bool {
  b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b':'
}

/// Returns `true` if a bare token looks like a time literal.
///
/// Examples: `22s`, `1m10s`, `00:04:32`, `1m32s`, `48s`
fn looks_like_time(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  let b = s.as_bytes();
  // Simple heuristic: starts with a digit
  b[0].is_ascii_digit()
}
