//! `aura sanitize` — pre-compiler source normalization.
//!
//! Scans `.aura` files for forbidden byte sequences (primarily escaped
//! quote characters like `\"`) and replaces them with semantically
//! equivalent Unicode alternatives **before** the zero-copy lexer
//! processes the file. This protects the AVX-2 hot path from human-
//! authored edge cases without requiring authors to memorize structural
//! constraints.
//!
//! # Replacements performed
//!
//! | Forbidden sequence | Unicode replacement | Code point |
//! | ------------------ | ------------------- | ---------- |
//! | `\"`               | `"` (left quote)    | U+201C     |
//! | `\'`               | `'` (left quote)    | U+2018     |
//! | `\n` (literal)     | space               | U+0020     |
//! | `\t` (literal)     | space               | U+0020     |
//! | lone `\` + other   | other (strip `\`)   | —          |
//!
//! # Integration with the version ledger
//!
//! Sanitized rewrites are normal file mutations. After running
//! `aura sanitize`, run `aura take` to record the clean state in
//! the history ledger. The ledger never records pre-sanitization
//! states, preserving immutability of the append-only store.
//!
//! # Usage
//!
//! ```sh
//! aura sanitize                      # normalize all files in cwd
//! aura sanitize --project ./my-proj  # specific project root
//! aura sanitize --dry-run            # print proposed changes, no writes
//! aura sanitize --path info/people.aura  # single file
//! ```

use crate::cfg::IgnoreList;
use crate::error::{CompileError, Result};
use crate::logs::Logger;
use std::path::{Path, PathBuf};

// -------------------------------------------------------------------- //
// Public API

/// Options for `aura sanitize`.
#[derive(Debug, Default)]
pub struct SanitizeOpts {
  /// Project root directory.
  pub project: PathBuf,
  /// If true, print proposed changes but do not write any files.
  pub dry_run: bool,
  /// If set, only sanitize this single file (relative to project root).
  pub path: Option<PathBuf>,
}

/// Runs `aura sanitize` over the project (or a single file).
pub fn run(opts: &SanitizeOpts) -> Result<()> {
  let log = Logger::new();

  if opts.dry_run {
    log.compile("Scanning for forbidden byte sequences (dry run)...");
  } else {
    log.compile("Sanitizing source files...");
  }

  let files: Vec<PathBuf> = match &opts.path {
    Some(rel) => {
      let abs = opts.project.join(rel);
      if !abs.exists() {
        return Err(CompileError::msg(format!(
          "file not found: {}",
          abs.display()
        )));
      }
      vec![abs]
    }
    None => {
      let ignore = IgnoreList::load(&opts.project)?;
      collect_aura_files(&opts.project, &ignore)?
    }
  };

  let mut changed = 0usize;

  for file in &files {
    let rel = file
      .strip_prefix(&opts.project)
      .unwrap_or(file.as_path())
      .display()
      .to_string();

    let src = std::fs::read_to_string(file)
      .map_err(|e| CompileError::msg(format!("cannot read {}: {}", file.display(), e)))?;

    let normalized = normalize(&src);

    if normalized == src {
      continue;
    }

    changed += 1;

    if opts.dry_run {
      log.warn(&rel, 0, None, "forbidden sequences detected", None);
      print_diff(&src, &normalized, &rel);
    } else {
      std::fs::write(file, normalized.as_bytes())
        .map_err(|e| CompileError::msg(format!("cannot write {}: {}", file.display(), e)))?;
      log.info(&format!("normalized {}", rel));
    }
  }

  let total = files.len();
  if changed == 0 {
    log.success(&format!(
      "{} file(s) scanned — no forbidden sequences found",
      total
    ));
  } else if opts.dry_run {
    log.success(&format!(
      "{} file(s) scanned — {} would be normalized (dry run, no writes)",
      total, changed
    ));
  } else {
    log.success(&format!(
      "{} file(s) scanned — {} normalized in place",
      total, changed
    ));
  }

  Ok(())
}

// -------------------------------------------------------------------- //
// Core normalization

/// Replaces forbidden byte sequences with valid Unicode equivalents.
///
/// This is the single O(n) forward pass that the pre-compiler sanitizer
/// runs. It never allocates a second buffer beyond the output `String`
/// and never needs to re-scan the input.
///
/// After this pass the resulting string is guaranteed to contain no
/// ASCII backslash escape sequences, allowing the zero-copy lexer to
/// process the file without exception paths.
pub fn normalize(src: &str) -> String {
  let bytes = src.as_bytes();
  let mut out: Vec<u8> = Vec::with_capacity(src.len() + 32);
  let mut i = 0;

  while i < bytes.len() {
    if bytes[i] == b'\\' && i + 1 < bytes.len() {
      match bytes[i + 1] {
        // \" → U+201C LEFT DOUBLE QUOTATION MARK
        b'"' => {
          out.extend_from_slice("\u{201C}".as_bytes());
          i += 2;
        }
        // \' → U+2018 LEFT SINGLE QUOTATION MARK
        b'\'' => {
          out.extend_from_slice("\u{2018}".as_bytes());
          i += 2;
        }
        // Literal \n or \t → space (safe content byte)
        b'n' | b't' => {
          out.push(b' ');
          i += 2;
        }
        // Any other backslash sequence → strip the backslash, keep char.
        _ => {
          i += 1; // skip backslash
        }
      }
    } else {
      out.push(bytes[i]);
      i += 1;
    }
  }

  // SAFETY: input is valid UTF-8 and our replacements are valid UTF-8.
  // The from_utf8_lossy fallback handles any edge case gracefully.
  String::from_utf8(out).unwrap_or_else(|e| {
    String::from_utf8_lossy(e.as_bytes()).into_owned()
  })
}

// -------------------------------------------------------------------- //
// Diff display

fn print_diff(original: &str, normalized: &str, file: &str) {
  let orig_lines: Vec<&str> = original.lines().collect();
  let norm_lines: Vec<&str> = normalized.lines().collect();

  for (n, (a, b)) in orig_lines.iter().zip(norm_lines.iter()).enumerate() {
    if a != b {
      eprintln!("  {}:{}: - {}", file, n + 1, a.trim_end());
      eprintln!("  {}:{}: + {}", file, n + 1, b.trim_end());
    }
  }
}

// -------------------------------------------------------------------- //
// File collection

fn collect_aura_files(root: &Path, ignore: &IgnoreList) -> Result<Vec<PathBuf>> {
  let mut files = Vec::new();
  collect_recursive(root, root, ignore, &mut files)?;
  files.sort();
  Ok(files)
}

fn collect_recursive(
  root: &Path,
  dir: &Path,
  ignore: &IgnoreList,
  out: &mut Vec<PathBuf>,
) -> Result<()> {
  let entries = std::fs::read_dir(dir)
    .map_err(|e| CompileError::msg(format!("cannot read {}: {}", dir.display(), e)))?;

  for entry in entries.flatten() {
    let path = entry.path();
    let rel = path.strip_prefix(root).unwrap_or(&path);
    if ignore.is_excluded(rel) {
      continue;
    }
    if path.is_dir() {
      collect_recursive(root, &path, ignore, out)?;
    } else if path.extension().and_then(|e| e.to_str()) == Some("aura") {
      out.push(path);
    }
  }
  Ok(())
}
