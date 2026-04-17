//! AURA compiler log formatter.
//!
//! Builds the final printable string for each log entry.
//! No heap allocation beyond the returned `String`.

use crate::logs::colors;


// -------------------------------------------------------------------- //
// Formatter

/// Format a log message into a coloured, human-readable string.
///
/// Layout:
/// ```text
/// COMPILE  Building project…
/// ERROR    credits.aura:5  expected `->`, got Comma
/// WARN     moods.aura:1  [W003] node `moods` has no `time` field
/// ```
///
/// # Arguments
/// * `kind`    — log kind string (`"compile"`, `"error"`, `"warn"`, …)
/// * `message` — primary message text
/// * `detail`  — optional secondary detail (file path, hint, etc.)
pub fn format_message(kind: &str, message: &str, detail: Option<&str>) -> String {
  let kind_color = colors::for_kind(kind);
  let label      = kind.to_uppercase();

  // Add the :: prefix for primary phases (COMPILE, SUCCESS)
  let prefix = if kind == "compile" || kind == "success" {
    format!("{}::{} ", colors::BOLD, colors::RESET)
  } else {
    "".to_string()
  };

  let kind_part  = format!("{}{:<8}{}", kind_color, label, colors::RESET);

  // AURA Sigils based on kind
  let sigil = match kind {
    "parse" | "emit" | "lint" => format!("{}->{} ", colors::DIM, colors::RESET),
    "success" | "error"       => format!("{}!!{} ", colors::BOLD, colors::RESET),
    _                         => "".to_string(),
  };

  let mut out = format!("{prefix}{kind_part} {sigil}{message}");

  if let Some(d) = detail {
    if !d.is_empty() {
      // Indent detail lines to match the prefix + label width
      let indent = if prefix.is_empty() { 9 } else { 12 };
      out.push_str(&format!(
        "\n{:indent$}{}  {}{}",
        "",
        colors::DIM,
        d,
        colors::RESET,
        indent = indent
      ));
    }
  }

  out
}

/// Format a compiler diagnostic line.
///
/// Layout:
/// ```text
/// ERROR    credits.aura:5  expected `->` at line 5, got Comma
///          ^------- bold file:line prefix
/// ```
pub fn format_diag(
  level: &str,
  file: &str,
  line: u32,
  code: Option<&str>,
  message: &str,
  hint: Option<&str>,
) -> String {
  let kind_color  = colors::for_kind(level);
  let label       = level.to_uppercase();

  let kind_part   = format!("{}{:<8}{}", kind_color, label, colors::RESET);
  let loc         = format!("{}{}:{}{}", colors::BOLD, file, line, colors::RESET);
  let code_part   = code
    .map(|c| format!(" {}[{}]{}", colors::DIM, c, colors::RESET))
    .unwrap_or_default();

  let mut out = format!("{kind_part} {}!!{} {loc}{code_part}  {message}", colors::BOLD, colors::RESET);

  if let Some(h) = hint {
    if !h.is_empty() {
      out.push_str(&format!(
        "\n         {}hint:{} {h}{}",
        colors::BRIGHT_CYAN, colors::RESET, colors::RESET
      ));
    }
  }

  out
}
