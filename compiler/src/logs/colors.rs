//! AURA compiler log colours.
//!
//! Pure-Rust ANSI escape constants — zero external dependencies.
//! Every constant is a `&'static str` so it compiles to a read-only
//! string literal with no runtime allocation.

// -------------------------------------------------------------------- //
// Reset

/// Resets all SGR attributes.
pub const RESET: &str = "\x1b[0m";

// -------------------------------------------------------------------- //
// Foreground colours (standard + bright)

pub const GRAY:          &str = "\x1b[90m";
pub const RED:           &str = "\x1b[31m";
pub const GREEN:         &str = "\x1b[32m";
pub const YELLOW:        &str = "\x1b[33m";
pub const BLUE:          &str = "\x1b[34m";
pub const MAGENTA:       &str = "\x1b[35m";
pub const CYAN:          &str = "\x1b[36m";
pub const WHITE:         &str = "\x1b[37m";

pub const BRIGHT_RED:    &str = "\x1b[91m";
pub const BRIGHT_GREEN:  &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE:   &str = "\x1b[94m";
pub const BRIGHT_MAGENTA:&str = "\x1b[95m";
pub const BRIGHT_CYAN:   &str = "\x1b[96m";
pub const BRIGHT_WHITE:  &str = "\x1b[97m";

// -------------------------------------------------------------------- //
// SGR modifiers

pub const BOLD:       &str = "\x1b[1m";
pub const DIM:        &str = "\x1b[2m";
pub const ITALIC:     &str = "\x1b[3m";
pub const UNDERLINE:  &str = "\x1b[4m";

// -------------------------------------------------------------------- //
// Compound styles — AURA-specific log kinds

/// `aura compile` phase — bold cyan
pub const COMPILE: &str   = "\x1b[1m\x1b[96m";
/// Lexer phase — dim cyan
pub const LEX: &str        = "\x1b[2m\x1b[36m";
/// Parser phase — cyan
pub const PARSE: &str      = "\x1b[36m";
/// Lint phase — bright yellow
pub const LINT: &str       = "\x1b[93m";
/// Emitter phase — bright blue
pub const EMIT: &str       = "\x1b[94m";
/// Info — blue
pub const INFO: &str       = "\x1b[34m";
/// Debug — gray
pub const DEBUG: &str      = "\x1b[90m";
/// Note — magenta
pub const NOTE: &str       = "\x1b[35m";
/// Warning — bold yellow
pub const WARN: &str       = "\x1b[1m\x1b[33m";
/// Error — bold red
pub const ERROR: &str      = "\x1b[1m\x1b[31m";
/// Success — bold green
pub const SUCCESS: &str    = "\x1b[1m\x1b[32m";

// -------------------------------------------------------------------- //
// Helpers

/// Return the colour constant for a given log-kind string.
/// Falls back to an empty string (no colour) for unknown kinds.
pub fn for_kind(kind: &str) -> &'static str {
  match kind {
    "compile" => COMPILE,
    "lex"     => LEX,
    "parse"   => PARSE,
    "lint"    => LINT,
    "emit"    => EMIT,
    "info"    => INFO,
    "debug"   => DEBUG,
    "note"    => NOTE,
    "warn"    => WARN,
    "warning" => WARN,
    "error"   => ERROR,
    "success" => SUCCESS,
    _         => "",
  }
}
