//! Time expression normalizer.
//!
//! Converts all AURA time syntaxes into an explicit `Interval` triple
//! `[low, high, duration]` and enforces the invariant:
//!
//! ```text
//! low + duration == high
//! ```
//!
//! If all three values are provided and violate the invariant, the
//! normalizer raises a hard `CompileError`. If only two are provided
//! the third is derived.
//!
//! # Supported syntaxes
//!
//! | Syntax              | Example          | Derives    |
//! | ------------------- | ---------------- | ---------- |
//! | `start~end`         | `22s~1m10s`      | duration   |
//! | `start+duration`    | `22s+48s`        | end        |
//! | `[start, end, dur]` | `[22s,1m10s,48s]`| none/check |
//! | `@time/value`       | `@time/1m32s`    | point→0dur |
//! | `HH:MM:SS~HH:MM:SS` | `00:04:32~00:07:18` | duration |

use crate::error::{CompileError, Result};
use crate::parse::ast::TimeExpr;
use aura::interval::Interval;

/// Parses and normalizes a `TimeExpr` into an `Interval`.
///
/// All parsing is done on the `&str` slices already produced by the
/// lexer — no additional allocation required.
pub struct TimeNorm;

impl TimeNorm {
  /// Converts a `TimeExpr` into a normalized `Interval`.
  pub fn normalize(expr: &TimeExpr<'_>) -> Result<Interval> {
    match expr {
      TimeExpr::Range { start, end } => {
        let low = parse_seconds(start)?;
        let high = parse_seconds(end)?;
        if high < low {
          return Err(CompileError::msg(format!(
            "time range end ({}) is before start ({}) in `{}~{}`",
            high, low, start, end
          )));
        }
        Ok(Interval::from_range(low, high))
      }

      TimeExpr::Offset { start, dur } => {
        let low = parse_seconds(start)?;
        let duration = parse_seconds(dur)?;
        Ok(Interval::from_start_dur(low, duration))
      }

      TimeExpr::Triple { start, end, dur } => {
        let low = parse_seconds(start)?;
        let high = parse_seconds(end)?;
        let duration = parse_seconds(dur)?;
        Interval::from_triple(low, high, duration).map_err(|e| {
          CompileError::msg(format!("time invariant violated in explicit triple: {}", e))
        })
      }

      TimeExpr::Anchor(t) => {
        let low = parse_seconds(t)?;
        Ok(Interval::point(low))
      }
    }
  }
}

/// Parses a time string into seconds as an `f32`.
///
/// Accepted formats:
/// - `Ns` — N seconds, e.g. `22s`, `48s`
/// - `NmMs` — N minutes M seconds, e.g. `1m10s`, `2m30s`
/// - `Nh` — N hours
/// - `HH:MM:SS` — film timestamp, e.g. `00:04:32`
/// - `HH:MM:SS.mmm` — film with milliseconds
/// - Plain integer/float — treated as seconds
pub fn parse_seconds(s: &str) -> Result<f32> {
  let s = s.trim();

  // HH:MM:SS or HH:MM:SS.mmm
  if s.contains(':') {
    return parse_hms(s);
  }

  // NmMs — minutes and seconds
  if let Some(m_pos) = s.find('m') {
    let mins: f32 = s[..m_pos]
      .parse()
      .map_err(|_| CompileError::msg(format!("invalid time literal: `{}`", s)))?;
    let rest = &s[m_pos + 1..];
    let secs: f32 = if rest.ends_with('s') && !rest.is_empty() {
      rest[..rest.len() - 1].parse().unwrap_or(0.0)
    } else if rest.is_empty() {
      0.0
    } else {
      return Err(CompileError::msg(format!("invalid time literal: `{}`", s)));
    };
    return Ok(mins * 60.0 + secs);
  }

  // Nh — hours
  if let Some(h_pos) = s.find('h') {
    let hours: f32 = s[..h_pos]
      .parse()
      .map_err(|_| CompileError::msg(format!("invalid time literal: `{}`", s)))?;
    return Ok(hours * 3600.0);
  }

  // Ns — seconds
  if s.ends_with('s') {
    let n: f32 = s[..s.len() - 1]
      .parse()
      .map_err(|_| CompileError::msg(format!("invalid time literal: `{}`", s)))?;
    return Ok(n);
  }

  // Nms — milliseconds
  if s.ends_with("ms") {
    let n: f32 = s[..s.len() - 2]
      .parse()
      .map_err(|_| CompileError::msg(format!("invalid time literal: `{}`", s)))?;
    return Ok(n / 1000.0);
  }

  // Plain number — treat as seconds
  s.parse()
    .map_err(|_| CompileError::msg(format!("invalid time literal: `{}`", s)))
}

fn parse_hms(s: &str) -> Result<f32> {
  // Split on ':' — expect 3 parts: HH MM SS[.mmm]
  let parts: Vec<&str> = s.splitn(3, ':').collect();
  if parts.len() != 3 {
    return Err(CompileError::msg(format!(
      "invalid HH:MM:SS time literal: `{}`",
      s
    )));
  }
  let h: f32 = parts[0]
    .parse()
    .map_err(|_| CompileError::msg(format!("invalid hours in `{}`", s)))?;
  let m: f32 = parts[1]
    .parse()
    .map_err(|_| CompileError::msg(format!("invalid minutes in `{}`", s)))?;
  let sec_str = parts[2];
  let sec: f32 = sec_str
    .parse()
    .map_err(|_| CompileError::msg(format!("invalid seconds in `{}`", s)))?;
  Ok(h * 3600.0 + m * 60.0 + sec)
}
