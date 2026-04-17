//! AURA compiler log formatter.
//!
//! Builds the final printable string for each log entry.
//! No heap allocation beyond the returned `String`.

use crate::logs::colors;
use std::time::{SystemTime, UNIX_EPOCH};

// -------------------------------------------------------------------- //
// Timestamp

/// Returns a compact UTC timestamp string: `HH:MM:SS`.
///
/// Uses only `std::time` — no `chrono` dependency.
fn timestamp() -> String {
  let secs = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|d| d.as_secs())
    .unwrap_or(0);

  let s = secs % 60;
  let m = (secs / 60) % 60;
  let h = (secs / 3600) % 24;
  format!("{h:02}:{m:02}:{s:02}")
}

// -------------------------------------------------------------------- //
// Formatter

/// Format a log message into a coloured, human-readable string.
///
/// Layout:
/// ```text
/// [HH:MM:SS] COMPILE  Building project…
/// [HH:MM:SS] ERROR    credits.aura:5  expected `->`, got Comma
/// [HH:MM:SS] WARN     moods.aura:1  [W003] node `moods` has no `time` field
/// ```
///
/// # Arguments
/// * `kind`    — log kind string (`"compile"`, `"error"`, `"warn"`, …)
/// * `message` — primary message text
/// * `detail`  — optional secondary detail (file path, hint, etc.)
pub fn format_message(kind: &str, message: &str, detail: Option<&str>) -> String {
  let kind_color = colors::for_kind(kind);
  let label      = kind.to_uppercase();
  let ts         = timestamp();

  let ts_part    = format!("{}[{}]{}", colors::GRAY, ts, colors::RESET);
  let kind_part  = format!("{}{:<8}{}", kind_color, label, colors::RESET);

  let mut out = format!("{ts_part} {kind_part} {message}");

  if let Some(d) = detail {
    if !d.is_empty() {
      out.push_str(&format!("\n         {}  {}{}", colors::DIM, d, colors::RESET));
    }
  }

  out
}

/// Format a compiler diagnostic line.
///
/// Layout:
/// ```text
/// [HH:MM:SS] ERROR    credits.aura:5  expected `->` at line 5, got Comma
///            ^------- bold file:line prefix
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
  let ts          = timestamp();
  let label       = level.to_uppercase();

  let ts_part     = format!("{}[{}]{}", colors::GRAY, ts, colors::RESET);
  let kind_part   = format!("{}{:<8}{}", kind_color, label, colors::RESET);
  let loc         = format!("{}{}:{}{}", colors::BOLD, file, line, colors::RESET);
  let code_part   = code
    .map(|c| format!(" {}[{}]{}", colors::DIM, c, colors::RESET))
    .unwrap_or_default();

  let mut out = format!("{ts_part} {kind_part} {loc}{code_part}  {message}");

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
