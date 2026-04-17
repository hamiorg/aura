//! Recursive-descent AURA parser.
//!
//! Consumes a `Scanner` token stream and produces a `Document` AST.
//!
//! # Indentation strategy
//!
//! The scanner emits `Token { kind, offset, line }` where `offset` is the
//! byte position in the source buffer. Column is computed on demand:
//!
//! ```text
//! col(src, offset) = offset - (position of last '\n' before offset) - 1
//! ```
//!
//! A namespace block opened by `name::` at column `c` owns all following
//! tokens at column `> c`. The block ends (without consuming) the first
//! non-whitespace token at column `<= c`.
//!
//! # Grammar (simplified)
//!
//! ```text
//! document  ::= (namespace | skip)*  EOF
//! namespace ::= KEY '::'  child*
//! child     ::= namespace | field | inherits | annotation | divider
//! field     ::= KEY marker? '->'  value ('|' value)*
//! inherits  ::= '>>'  ref
//! value     ::= QUOTED | time_expr | KEY | ref | list
//! time_expr ::= TIME ('~' TIME | '+' TIME)?
//!             | KEY  ('~' TIME | '+' TIME)
//! list      ::= '[' value (',' value)* ']'
//! ```

use crate::error::{CompileError, Result, Span};
use crate::lex::scan::Scanner;
use crate::lex::token::{Kind, Token};
use crate::parse::ast::{
  Child, Document, Field, FieldMarker, Namespace, NodeType, RefBody, Reference, TimeExpr, Value,
};
use std::collections::VecDeque;

// -------------------------------------------------------------------- //
// Column helper

fn col(src: &str, offset: usize) -> usize {
  let safe = offset.min(src.len());
  match src[..safe].rfind('\n') {
    Some(pos) => safe.saturating_sub(pos + 1),
    None => safe,
  }
}

fn span_of(tok: &Token<'_>) -> Span {
  Span::new(tok.offset, tok.offset + 1, tok.line, 0)
}

// -------------------------------------------------------------------- //
// Parser — two-token lookahead via a small queue

/// Recursive-descent AURA parser.
pub struct Parser<'src> {
  scanner: Scanner<'src>,
  src: &'src str,
  /// Two-token lookahead buffer. Filled lazily.
  buf: VecDeque<Token<'src>>,
}

impl<'src> Parser<'src> {
  pub fn new(src: &'src str) -> Self {
    Self {
      scanner: Scanner::new(src),
      src,
      buf: VecDeque::with_capacity(2),
    }
  }

  /// Parse the full source text into a `Document`.
  pub fn parse(&mut self) -> Result<Document<'src>> {
    let mut doc = Document {
      namespaces: Vec::new(),
      path: None,
    };
    loop {
      self.skip_ws();
      if self.is_eof() {
        break;
      }
      if self.is_namespace_opener() {
        doc.namespaces.push(self.parse_namespace(0)?);
      } else {
        self.eat()?; // skip unrecognised top-level token
      }
    }
    Ok(doc)
  }

  // ---------------------------------------------------------------- //
  // Namespace parsing

  fn parse_namespace(&mut self, parent_col: usize) -> Result<Namespace<'src>> {
    let key_tok = self.eat()?;
    let name = match key_tok.kind {
      Kind::Key(s) => s,
      _ => return Err(CompileError::msg("expected namespace name")),
    };
    let ns_col = col(self.src, key_tok.offset);
    let span = span_of(&key_tok);

    // Consume `::`
    self.eat_expect(|k| matches!(k, Kind::ScopeOpen), "`::`")?;

    let node_type = NodeType::from_name(name);
    let raw_slug = name.starts_with('$');
    let mut children: Vec<Child<'src>> = Vec::new();

    loop {
      self.skip_ws();
      if self.is_eof() {
        break;
      }

      let tok_col = self.peek_col();

      // Dedent — block ends.
      if tok_col <= ns_col {
        // Edge: if the namespace itself is at col 0 and we're looking at
        // another top-level Key::, we should stop.
        if tok_col <= parent_col || (tok_col == ns_col && !children.is_empty()) {
          break;
        }
        // Same column as opener with no children yet: empty block, done.
        if tok_col == ns_col {
          break;
        }
      }

      match self.peek0_kind() {
        Kind::Key(_) => {
          if self.is_namespace_opener() {
            children.push(Child::Block(self.parse_namespace(ns_col)?));
          } else {
            children.push(Child::Field(self.parse_field()?));
          }
        }
        Kind::Inherits => {
          children.push(Child::Field(self.parse_inherits()?));
        }
        Kind::AnnotationText(_) | Kind::Divider | Kind::Newline => {
          self.eat()?;
        }
        _ => {
          self.eat()?;
        }
      }
    }

    Ok(Namespace {
      name,
      path: name.to_string(),
      children,
      span,
      node_type,
      raw_slug,
    })
  }

  // ---------------------------------------------------------------- //
  // Field parsing

  fn parse_field(&mut self) -> Result<Field<'src>> {
    let key_tok = self.eat()?;
    let key = match key_tok.kind {
      Kind::Key(s) => s,
      _ => "",
    };
    let span = span_of(&key_tok);

    let marker = match self.peek0_kind() {
      Kind::Required => {
        self.eat()?;
        Some(FieldMarker::Required)
      }
      Kind::Optional => {
        self.eat()?;
        Some(FieldMarker::Optional)
      }
      Kind::Custom => {
        self.eat()?;
        Some(FieldMarker::Custom)
      }
      _ => None,
    };

    self.eat_expect(|k| matches!(k, Kind::Arrow), "`->`")?;
    let value = self.parse_value_union()?;
    Ok(Field {
      key,
      value,
      marker,
      span,
    })
  }

  fn parse_inherits(&mut self) -> Result<Field<'src>> {
    let tok = self.eat()?; // consume >>
    let span = span_of(&tok);
    let val = self.parse_single_value()?;
    let r = match val {
      Value::Ref(r) => r,
      _ => Reference {
        domain: "",
        body: RefBody::Single(""),
        span,
      },
    };
    Ok(Field {
      key: ">>",
      value: Value::Inherits(r),
      marker: None,
      span,
    })
  }

  // ---------------------------------------------------------------- //
  // Value parsing

  fn parse_value_union(&mut self) -> Result<Value<'src>> {
    let first = self.parse_single_value()?;
    if matches!(self.peek0_kind(), Kind::Pipe) {
      let mut parts = vec![first];
      while matches!(self.peek0_kind(), Kind::Pipe) {
        self.eat()?;
        parts.push(self.parse_single_value()?);
      }
      return Ok(Value::Union(parts));
    }
    Ok(first)
  }

  fn parse_single_value(&mut self) -> Result<Value<'src>> {
    match self.peek0_kind() {
      Kind::Quoted(s) => {
        self.eat()?;
        Ok(Value::Str(s))
      }

      Kind::Time(t) => {
        self.eat()?;
        self.finish_time(t)
      }

      Kind::Key(k) => {
        self.eat()?;
        // Bare key can start a time expression.
        match self.peek0_kind() {
          Kind::Tilde => {
            self.eat()?;
            let e = self.eat_time_str()?;
            Ok(Value::Time(TimeExpr::Range { start: k, end: e }))
          }
          Kind::Plus => {
            self.eat()?;
            let d = self.eat_time_str()?;
            Ok(Value::Time(TimeExpr::Offset { start: k, dur: d }))
          }
          _ => Ok(Value::Bare(k)),
        }
      }

      Kind::RefAt => {
        let at = self.eat()?;
        let span = span_of(&at);
        let path_tok = self.eat()?;
        let path_str = match path_tok.kind {
          Kind::RefPath(p) => p,
          Kind::Key(k) => k,
          _ => "",
        };
        // @time/value → time anchor
        if let Some(rest) = path_str.strip_prefix("time/") {
          return Ok(Value::Time(TimeExpr::Anchor(rest)));
        }
        // @domain/[id1, id2] — inline bracket list.
        //
        // The scanner stops at `[`, so for `@people/[p9gregk, p8paule]` the
        // RefPath token is `"people/"` (domain with trailing slash, empty body).
        // When the next token is `[` we parse the ID list here and build the
        // reference ourselves instead of delegating to `build_ref`.
        if path_str.ends_with('/') && matches!(self.peek0_kind(), Kind::BracketOpen) {
          self.eat()?; // consume `[`
          // Strip the trailing `/` to get the plain domain name.
          let domain = &path_str[..path_str.len() - 1];
          let mut ids: Vec<&'src str> = Vec::new();
          loop {
            self.skip_ws();
            if matches!(self.peek0_kind(), Kind::BracketClose | Kind::Eof) {
              break;
            }
            // Each element is a bare ID key (possibly hyphenated, e.g. `main-artist`).
            let item = self.eat()?;
            let id: &'src str = match item.kind {
              Kind::Key(k) => k,
              Kind::RefPath(p) => p,
              Kind::Bare(b) => b,
              _ => "",
            };
            if !id.is_empty() {
              ids.push(id);
            }
            self.skip_ws();
            if matches!(self.peek0_kind(), Kind::Comma) {
              self.eat()?; // consume `,`
            } else {
              break;
            }
          }
          if matches!(self.peek0_kind(), Kind::BracketClose) {
            self.eat()?; // consume `]`
          }
          return Ok(Value::Ref(Reference {
            domain,
            body: RefBody::List(ids),
            span,
          }));
        }
        Ok(Value::Ref(build_ref(path_str, span)))
      }

      Kind::RefPath(p) => {
        let tok = self.eat()?;
        let span = span_of(&tok);
        if let Some(rest) = p.strip_prefix("time/") {
          return Ok(Value::Time(TimeExpr::Anchor(rest)));
        }
        Ok(Value::Ref(build_ref(p, span)))
      }

      Kind::BracketOpen => {
        self.eat()?;
        self.parse_list()
      }

      _ => {
        // Unknown value — return bare empty so parsing continues.
        self.eat()?;
        Ok(Value::Bare(""))
      }
    }
  }

  fn finish_time(&mut self, start: &'src str) -> Result<Value<'src>> {
    match self.peek0_kind() {
      Kind::Tilde => {
        self.eat()?;
        let e = self.eat_time_str()?;
        Ok(Value::Time(TimeExpr::Range { start, end: e }))
      }
      Kind::Plus => {
        self.eat()?;
        let d = self.eat_time_str()?;
        Ok(Value::Time(TimeExpr::Offset { start, dur: d }))
      }
      _ => Ok(Value::Time(TimeExpr::Anchor(start))),
    }
  }

  fn eat_time_str(&mut self) -> Result<&'src str> {
    let tok = self.eat()?;
    Ok(match tok.kind {
      Kind::Time(t) => t,
      Kind::Key(k) => k,
      _ => "",
    })
  }

  fn parse_list(&mut self) -> Result<Value<'src>> {
    let mut items = Vec::new();
    loop {
      self.skip_ws();
      if matches!(self.peek0_kind(), Kind::BracketClose | Kind::Eof) {
        break;
      }
      items.push(self.parse_single_value()?);
      self.skip_ws();
      if matches!(self.peek0_kind(), Kind::Comma) {
        self.eat()?;
      } else {
        break;
      }
    }
    if matches!(self.peek0_kind(), Kind::BracketClose) {
      self.eat()?;
    }

    // Detect time triple: [time, time, time]
    if items.len() == 3 {
      let (s, e, d) = match (&items[0], &items[1], &items[2]) {
        (
          Value::Time(TimeExpr::Anchor(s)) | Value::Bare(s),
          Value::Time(TimeExpr::Anchor(e)) | Value::Bare(e),
          Value::Time(TimeExpr::Anchor(d)) | Value::Bare(d),
        ) if looks_like_time(s) && looks_like_time(e) && looks_like_time(d) => (*s, *e, *d),
        _ => ("", "", ""),
      };
      if !s.is_empty() {
        return Ok(Value::Time(TimeExpr::Triple {
          start: s,
          end: e,
          dur: d,
        }));
      }
    }

    Ok(Value::List(items))
  }

  // ---------------------------------------------------------------- //
  // Low-level token operations

  /// Fill the buffer to at least `n` tokens (or until EOF).
  fn fill(&mut self, n: usize) -> Result<()> {
    while self.buf.len() < n {
      let t = self.scanner.next()?;
      let eof = matches!(t.kind, Kind::Eof);
      self.buf.push_back(t);
      if eof {
        break;
      }
    }
    Ok(())
  }

  /// Peek at the token at index `i` in the buffer (0-indexed).
  #[allow(dead_code)]
  fn peek_n(&mut self, i: usize) -> Result<Token<'src>> {
    self.fill(i + 1)?;
    Ok(self.buf.get(i).cloned().unwrap_or(Token {
      kind: Kind::Eof,
      offset: self.src.len(),
      line: 0,
    }))
  }

  fn peek0_kind(&mut self) -> Kind<'src> {
    self.fill(1).ok();
    self
      .buf
      .front()
      .map(|t| t.kind.clone())
      .unwrap_or(Kind::Eof)
  }

  fn peek_col(&mut self) -> usize {
    self.fill(1).ok();
    self
      .buf
      .front()
      .map(|t| col(self.src, t.offset))
      .unwrap_or(0)
  }

  fn eat(&mut self) -> Result<Token<'src>> {
    self.fill(1)?;
    Ok(self.buf.pop_front().unwrap_or(Token {
      kind: Kind::Eof,
      offset: self.src.len(),
      line: 0,
    }))
  }

  fn eat_expect<F: Fn(&Kind<'src>) -> bool>(
    &mut self,
    pred: F,
    expected: &str,
  ) -> Result<Token<'src>> {
    let tok = self.eat()?;
    if pred(&tok.kind) {
      Ok(tok)
    } else {
      Err(CompileError::msg(format!(
        "expected {} at line {}, got {:?}",
        expected, tok.line, tok.kind
      )))
    }
  }

  fn is_eof(&mut self) -> bool {
    matches!(self.peek0_kind(), Kind::Eof)
  }

  fn skip_ws(&mut self) {
    loop {
      match self.peek0_kind() {
        Kind::Newline | Kind::Divider | Kind::AnnotationText(_) => {
          self.fill(1).ok();
          self.buf.pop_front();
        }
        _ => break,
      }
    }
  }

  /// Returns true if the next two meaningful tokens are `Key` then `ScopeOpen`.
  fn is_namespace_opener(&mut self) -> bool {
    self.fill(2).ok();
    let k0 = self.buf.get(0).map(|t| &t.kind);
    let k1 = self.buf.get(1).map(|t| &t.kind);
    matches!(k0, Some(Kind::Key(_))) && matches!(k1, Some(Kind::ScopeOpen))
  }
}

// -------------------------------------------------------------------- //
// Reference builder

fn build_ref<'src>(path_str: &'src str, span: Span) -> Reference<'src> {
  if let Some(slash) = path_str.find('/') {
    let domain = &path_str[..slash];
    let body_str = &path_str[slash + 1..];

    if body_str.starts_with('[') {
      let inner = body_str.trim_start_matches('[').trim_end_matches(']');
      let ids: Vec<&'src str> = inner.split(',').map(str::trim).collect();
      return Reference {
        domain,
        body: RefBody::List(ids),
        span,
      };
    }
    if body_str.contains('/') {
      let parts: Vec<&'src str> = body_str.split('/').collect();
      return Reference {
        domain,
        body: RefBody::Path(parts),
        span,
      };
    }
    if domain.contains('.') {
      return Reference {
        domain: path_str,
        body: RefBody::Global(path_str),
        span,
      };
    }
    Reference {
      domain,
      body: RefBody::Single(body_str),
      span,
    }
  } else {
    Reference {
      domain: path_str,
      body: RefBody::Single(""),
      span,
    }
  }
}

fn looks_like_time(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }
  let b = s.as_bytes();
  b[0].is_ascii_digit()
}
