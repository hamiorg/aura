//! Token type emitted by the lexer.
//!
//! The AURA lexer emits a stream of `Token` values. Each token carries
//! a `&'a str` slice pointing directly into the source buffer — no heap
//! allocation, no copying.
//!
//! # Sigil vocabulary (complete)
//!
//! | Sigil  | Token variant  | Meaning                              |
//! | ------ | -------------- | ------------------------------------ |
//! | `::`   | `ScopeOpen`    | Opens a namespace block or node      |
//! | `->`   | `Arrow`        | Assigns a literal value to a key     |
//! | `@`    | `RefAt`        | References a named entity            |
//! | `##`   | `Annotation`   | Queryable comment compiled into HAMI |
//! | `--`   | `Divider`      | Visual separator (no compile output) |
//! | `\|`   | `Pipe`         | Union of values or cross-domain refs |
//! | `?`    | `Optional`     | Field may be absent                  |
//! | `!`    | `Required`     | Field must be present                |
//! | `~`    | `Tilde`        | Separates start~end in a time range  |
//! | `+`    | `Plus`         | Separates start+duration             |
//! | `[`    | `BracketOpen`  | Starts a list or time triple         |
//! | `]`    | `BracketClose` | Ends a list or time triple           |
//! | `,`    | `Comma`        | Separates list elements              |
//! | `>>`   | `Inherits`     | Extends a template or base schema    |
//! | `*`    | `Wildcard`     | Match all in a namespace query       |
//! | `$`    | (Key prefix)   | Vocab slug escape — raw vocabulary identifier |

/// A lexer token with a source slice and position.
///
/// The lifetime `'src` ties every `&str` slice back to the original
/// source buffer so no string data is ever copied during lexing.
#[derive(Debug, Clone, PartialEq)]
pub struct Token<'src> {
  pub kind: Kind<'src>,
  /// Byte offset of the first byte of this token in the source buffer.
  pub offset: usize,
  /// 1-indexed source line.
  pub line: u32,
}

/// The kind of a lexer token.
///
/// Structural tokens (sigils, indentation) carry no text slice.
/// Text tokens carry a `&'src str` pointing into the source buffer.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind<'src> {
  // --- Structural tokens (control codes `< 0x20` or compound sigils) ---
  /// `::` — scope opener / namespace jump.
  ScopeOpen,
  /// `->` — value assignment arrow.
  Arrow,
  /// `@` — reference sigil.
  RefAt,
  /// `##` — queryable annotation (compiled into HAMI).
  Annotation,
  /// `--` — visual divider (no compile output).
  Divider,
  /// `|` — union pipe.
  Pipe,
  /// `?` — optional field marker.
  Optional,
  /// `!` — required field marker.
  Required,
  /// `~` — range separator in `start~end` time syntax.
  Tilde,
  /// `+` — offset separator in `start+duration` time syntax.
  Plus,
  /// `[` — list or time triple open.
  BracketOpen,
  /// `]` — list or time triple close.
  BracketClose,
  /// `,` — list element separator.
  Comma,
  /// `>>` — inherits-from arc.
  Inherits,
  /// `*` — wildcard match.
  Wildcard,
  /// Significant indentation change: positive = indent, negative = dedent.
  Indent(i32),
  /// End of line (`\n`).
  Newline,
  /// End of file.
  Eof,

  // --- Text tokens (zero-copy slices into the source buffer) ---
  /// An unquoted key identifier, e.g. `name`, `verse`, `one`.
  Key(&'src str),
  /// A bare (unquoted) value, e.g. `1.0.0`, `2024-11-01`, `live`.
  Bare(&'src str),
  /// A quoted string value, e.g. `"Signal Loss"`.
  /// The slice includes the surrounding quotes.
  Quoted(&'src str),
  /// A time literal, e.g. `22s`, `1m10s`, `00:04:32`.
  Time(&'src str),
  /// A reference path after `@`, e.g. `person/p4xt9k2`.
  RefPath(&'src str),
  /// An annotation body after `##`.
  AnnotationText(&'src str),
}

impl<'src> Kind<'src> {
  /// Returns `true` if this token carries no compile output.
  pub fn is_whitespace(&self) -> bool {
    matches!(
      self,
      Self::Divider | Self::Newline | Self::Annotation | Self::AnnotationText(_)
    )
  }

  /// Returns `true` if this is a structural token (sigil or control).
  pub fn is_sigil(&self) -> bool {
    matches!(
      self,
      Self::ScopeOpen
        | Self::Arrow
        | Self::RefAt
        | Self::Pipe
        | Self::Optional
        | Self::Required
        | Self::Tilde
        | Self::Plus
        | Self::BracketOpen
        | Self::BracketClose
        | Self::Comma
        | Self::Inherits
        | Self::Wildcard
    )
  }
}
