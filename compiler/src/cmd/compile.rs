//! `aura compile`, `aura validate`, `aura lint` command handlers.

use crate::cfg::IgnoreList;
use crate::emit::{AtomEmitter, HamiEmitter};
use crate::error::{CompileError, Result};
use crate::hist::{DeltaReplayer, HistoryStore};
use crate::lint::Linter;
use crate::logs::Logger;
use crate::ns::NamespaceLoader;
use crate::parse::ast::Document;
use crate::parse::parse::Parser;
use crate::parse::resolve::Resolver;
use std::fs;
use std::path::{Path, PathBuf};

/// Options passed to the compile command.
#[derive(Debug, Default)]
pub struct CompileOpts {
  pub project: PathBuf,
  pub take: Option<String>,
  pub embed_history: bool,
  pub out_dir: Option<PathBuf>,
  pub strict: bool,
}

// -------------------------------------------------------------------- //
// compile

/// Runs the full `aura compile` pipeline.
///
/// Parses every `.aura` file in the project, merges all namespaces into
/// a single document, and emits ONE `.hami` + ONE `.atom` (if interval
/// nodes are present) to `dist/`.
pub fn run(opts: &CompileOpts) -> Result<()> {
  let out_dir = opts
    .out_dir
    .clone()
    .unwrap_or_else(|| opts.project.join("dist"));
  let log = Logger::new();
  log.compile("Building project...");

  // Namespace discovery.
  let mut ns_loader = NamespaceLoader::new(&opts.project);
  ns_loader.load()?;

  // Ignore list.
  let ignore = IgnoreList::load(&opts.project)?;

  // Optional history replay.
  if let Some(take_id) = &opts.take {
    let store = HistoryStore::open(&opts.project)?;
    let replayer = DeltaReplayer::new(&store);
    let state = replayer.reconstruct(take_id)?;
    log.info(&format!(
      "reconstructed {} nodes from take {}",
      state.len(),
      take_id
    ));
  }

  // Collect source files.
  let files = collect_aura_files(&opts.project, &ignore)?;
  if files.is_empty() {
    log.info(&format!(
      "no .aura files found in {}",
      opts.project.display()
    ));
    return Ok(());
  }

  fs::create_dir_all(&out_dir)
    .map_err(|e| CompileError::msg(format!("cannot create {}: {}", out_dir.display(), e)))?;

  // Parse all files and merge into a single document.
  let mut merged = Document {
    namespaces: Vec::new(),
    path: None,
  };
  let mut resolver = Resolver::new(opts.strict);
  let mut parse_errors = 0usize;

  for file in &files {
    let rel_path = file.strip_prefix(&opts.project).unwrap_or(file);
    let rel_str = rel_path.display().to_string();

    log.parse(&rel_str);
    match parse_file(file) {
      Err(e) => {
        log.error(&rel_str, 0, None, &e.to_string(), None);
        parse_errors += 1;
      }
      Ok(doc) => {
        let lint = Linter::new(opts.strict).lint(&doc, file);
        lint.print(Some(&opts.project));
        if lint.has_errors() {
          parse_errors += 1;
          continue;
        }
        resolver.register_document(&doc, file.to_path_buf());
        // Absorb this file's namespaces into the merged document.
        merged.namespaces.extend(doc.namespaces);
      }
    }
  }

  if parse_errors > 0 {
    return Err(CompileError::msg(format!(
      "{} file(s) had errors — compilation aborted",
      parse_errors
    )));
  }

  // Surface resolver warnings.
  if let Some(err) = resolver.into_error() {
    for diag in &err.diagnostics {
      let rel_file = diag.file.as_ref()
        .and_then(|p| p.strip_prefix(&opts.project).ok())
        .or(diag.file.as_deref());
      
      let file_str = rel_file.and_then(|p| p.to_str()).unwrap_or("");
      let line = diag.span.map(|s| s.line).unwrap_or(0);
      match diag.level {
        crate::error::Level::Warning => log.warn(file_str, line, None, &diag.message, diag.hint.as_deref()),
        crate::error::Level::Error => log.error(file_str, line, None, &diag.message, diag.hint.as_deref()),
        crate::error::Level::Note => log.note(&diag.message),
      }
    }
    if err.is_fatal() {
      return Err(err);
    }
  }

  // Determine output stem from the project namespace slug or folder name.
  let stem = project_stem(&opts.project);

  // Emit ONE .hami file (all manifests, credits, schema).
  let hami_bytes = HamiEmitter::new().emit(&merged)?;
  let hami_path = out_dir.join(format!("{}.hami", stem));
  fs::write(&hami_path, &hami_bytes)
    .map_err(|e| CompileError::msg(format!("cannot write {}: {}", hami_path.display(), e)))?;

  // Emit ONE .atom file only if the project has interval-indexed nodes.
  let mut atom = AtomEmitter::new();
  let atom_bytes = atom.emit(&merged)?;

  let output_line = if atom.node_count() > 0 {
    let atom_path = out_dir.join(format!("{}.atom", stem));
    fs::write(&atom_path, &atom_bytes)
      .map_err(|e| CompileError::msg(format!("cannot write {}: {}", atom_path.display(), e)))?;
    format!("{}.hami + {}.atom", stem, stem)
  } else {
    format!("{}.hami", stem)
  };

  log.success(&format!("{} file(s) → dist/{}", files.len(), output_line));
  Ok(())
}

// -------------------------------------------------------------------- //
// validate

/// Validate syntax and references without emitting output.
pub fn validate(project: &PathBuf, strict: bool) -> Result<()> {
  let log = Logger::new();
  let ignore = IgnoreList::load(project)?;
  let files = collect_aura_files(project, &ignore)?;
  let linter = Linter::new(strict);
  let mut errors = 0usize;
  for file in &files {
    let rel_path = file.strip_prefix(project).unwrap_or(file);
    let rel_str = rel_path.display().to_string();

    match parse_file(file) {
      Err(e) => {
        log.error(&rel_str, 0, None, &e.to_string(), None);
        errors += 1;
      }
      Ok(doc) => {
        let result = linter.lint(&doc, file);
        result.print(Some(project));
        if result.has_errors() {
          errors += 1;
        }
      }
    }
  }

  if errors > 0 {
    Err(CompileError::msg(format!("{} file(s) have errors", errors)))
  } else {
    log.success(&format!("{} file(s) ok", files.len()));
    Ok(())
  }
}

// -------------------------------------------------------------------- //
// lint

/// Run all lint rules and print diagnostics.
pub fn lint(project: &PathBuf, strict: bool) -> Result<()> {
  let log = Logger::new();
  let ignore = IgnoreList::load(project)?;
  let files = collect_aura_files(project, &ignore)?;
  let linter = Linter::new(strict);

  let mut total_diags = 0usize;
  let mut total_errs = 0usize;
  let mut parse_errs = 0usize;
  for file in &files {
    let rel_path = file.strip_prefix(project).unwrap_or(file);
    let rel_str = rel_path.display().to_string();

    match parse_file(file) {
      Err(e) => {
        log.error(&rel_str, 0, None, &e.to_string(), None);
        parse_errs += 1;
      }
      Ok(doc) => {
        let result = linter.lint(&doc, file);
        total_errs += result
          .diags
          .iter()
          .filter(|d| d.level == crate::error::Level::Error)
          .count();
        total_diags += result.diags.len();
        result.print(Some(project));
      }
    }
  }

  log.info(&format!(
    "{} file(s)  {} diagnostic(s)  {} error(s)",
    files.len(),
    total_diags,
    total_errs + parse_errs
  ));

  if total_errs + parse_errs > 0 {
    Err(CompileError::msg("lint found errors"))
  } else {
    Ok(())
  }
}

// -------------------------------------------------------------------- //
// Internal helpers

/// Parse a single `.aura` file into a `Document`.
fn parse_file(file: &Path) -> Result<Document<'static>> {
  // We need a 'static document for collecting into a merged doc.
  // Leak the source string so its lifetime is 'static.
  // This is safe because the process exits after compilation.
  let src = fs::read_to_string(file)
    .map_err(|e| CompileError::msg(format!("cannot read {}: {}", file.display(), e)))?;
  let src: &'static str = Box::leak(src.into_boxed_str());

  Parser::new(src)
    .parse()
    .map_err(|e| CompileError::msg(format!("parse error in {}: {}", file.display(), e)))
}

/// Derive the project output stem from the root `name.aura` entry.
///
/// Reads the `name::id ->` field from the root `name.aura` file.
/// Falls back to scanning the project root for any ID-named `.aura` file
/// (for backward compatibility with pre-v0.2 projects that still use
/// `namespace.aura` + a separate `{id}.aura` manifest).
/// Final fallback: the project folder name.
fn project_stem(project: &Path) -> String {
  // Try to read the id from the new name.aura entry point.
  let name_file = project.join("name.aura");
  if name_file.exists() {
    if let Ok(text) = fs::read_to_string(&name_file) {
      for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("id") {
          if let Some(arrow) = rest.trim_start().strip_prefix("->") {
            let id = arrow.trim().trim_matches('"');
            if !id.is_empty() {
              return id.to_string();
            }
          }
        }
      }
    }
  }

  // Backward compat: scan root for a non-name .aura file with an ID stem.
  if let Ok(entries) = fs::read_dir(project) {
    let mut candidates: Vec<String> = entries
      .flatten()
      .filter_map(|e| {
        let path = e.path();
        if !path.is_file() {
          return None;
        }
        if path.extension()?.to_str()? != "aura" {
          return None;
        }
        let stem = path.file_stem()?.to_str()?.to_string();
        // Exclude known non-manifest files.
        if stem == "name" || stem == "namespace" {
          return None;
        }
        Some(stem)
      })
      .collect();
    candidates.sort();
    if let Some(id) = candidates.into_iter().next() {
      return id;
    }
  }

  // Fallback: project folder name.
  project
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("project")
    .to_string()
}

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
  let entries = fs::read_dir(dir)
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
