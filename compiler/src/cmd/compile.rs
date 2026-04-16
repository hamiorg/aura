//! `aura compile`, `aura validate`, `aura lint` handlers.

use crate::cfg::IgnoreList;
use crate::emit::{AtomEmitter, HamiEmitter};
use crate::error::{CompileError, Result};
use crate::hist::{DeltaReplayer, HistoryStore};
use crate::ns::NamespaceLoader;
use crate::parse::resolve::Resolver;
use std::path::PathBuf;

/// Options passed to the compile command.
#[derive(Debug, Default)]
pub struct CompileOpts {
  /// Project root directory to compile.
  pub project: PathBuf,
  /// If set, compile a historical take rather than the working draft.
  pub take: Option<String>,
  /// If `true`, embed HistoryNodes (`0x14`) into the `.atom` output.
  pub embed_history: bool,
  /// Write output to this directory (default: `dist/`).
  pub out_dir: Option<PathBuf>,
  /// Treat unresolved references as errors (strict mode).
  pub strict: bool,
}

/// Runs the full `aura compile` pipeline.
///
/// 1. Load namespace symbol table.
/// 2. Load ignore list.
/// 3. If `--take`, replay delta chain to reconstruct virtual source.
/// 4. Lex + parse each `.aura` source file.
/// 5. Resolve references.
/// 6. Normalize time expressions.
/// 7. Expand `>>` arcs.
/// 8. Emit `.atom`, `.hami`, `.atlas` to `dist/`.
pub fn run(opts: &CompileOpts) -> Result<()> {
  let out_dir = opts
    .out_dir
    .clone()
    .unwrap_or_else(|| opts.project.join("dist"));

  // Step 1 — namespace discovery.
  let mut ns_loader = NamespaceLoader::new(&opts.project);
  ns_loader.load()?;

  // Step 2 — ignore list.
  let ignore = IgnoreList::load(&opts.project)?;

  // Step 3 — optional history replay.
  if let Some(take_id) = &opts.take {
    let store = HistoryStore::open(&opts.project)?;
    let replayer = DeltaReplayer::new(&store);
    let state = replayer.reconstruct(take_id)?;
    // `state` is a HashMap<path, aura_text>.
    // In the full implementation: feed state into the parse pipeline.
    // Scaffold: just confirm the reconstruction ran.
    eprintln!(
      "[compile] reconstructed {} nodes from take {}",
      state.len(),
      take_id
    );
  }

  // Steps 4–8 — per-file pipeline.
  // Walk all non-excluded `.aura` files under the project root.
  let mut resolver = Resolver::new(opts.strict);
  let mut atom_out = AtomEmitter::new();
  let mut hami_out = HamiEmitter::new();

  let files = collect_aura_files(&opts.project, &ignore)?;
  for file in &files {
    compile_one(file, &mut resolver, &mut atom_out, &mut hami_out, &out_dir)?;
  }

  // Surface accumulated warnings.
  if let Some(err) = resolver.into_error() {
    // Non-strict: warnings only. Print and continue.
    for diag in &err.diagnostics {
      eprintln!("{}", diag);
    }
    if err.is_fatal() {
      return Err(err);
    }
  }

  eprintln!(
    "[compile] done — {} files processed → {}",
    files.len(),
    out_dir.display()
  );
  Ok(())
}

/// Validates syntax and reference resolution without emitting output.
pub fn validate(project: &PathBuf, strict: bool) -> Result<()> {
  let opts = CompileOpts {
    project: project.clone(),
    strict,
    ..Default::default()
  };
  // Validation is compile with a /dev/null output dir.
  // In the full implementation, skip the emit step entirely.
  run(&opts)
}

/// Checks for style violations and best-practice warnings.
pub fn lint(project: &PathBuf) -> Result<()> {
  // Lint rules (to be implemented):
  // - Keys not in the standard key list → warning
  // - Boolean fields using `true`/`false` instead of `live`/`dark`
  // - Missing `screen` field on person nodes
  // - `thumbnail` or `artwork` keys used (both removed)
  eprintln!("[lint] project: {}", project.display());
  Ok(())
}

// -------------------------------------------------------------------- //
// Internal helpers

fn collect_aura_files(root: &std::path::Path, ignore: &IgnoreList) -> Result<Vec<PathBuf>> {
  let mut files = Vec::new();
  collect_recursive(root, root, ignore, &mut files)?;
  files.sort();
  Ok(files)
}

fn collect_recursive(
  root: &std::path::Path,
  dir: &std::path::Path,
  ignore: &IgnoreList,
  out: &mut Vec<PathBuf>,
) -> Result<()> {
  let entries = std::fs::read_dir(dir)
    .map_err(|e| CompileError::msg(format!("cannot read directory `{}`: {}", dir.display(), e)))?;
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

fn compile_one(
  file: &std::path::Path,
  _resolver: &mut Resolver,
  _atom_out: &mut AtomEmitter,
  _hami_out: &mut HamiEmitter,
  _out_dir: &std::path::Path,
) -> Result<()> {
  // In the full implementation:
  // 1. Read file bytes
  // 2. Scanner::new(src) → token stream
  // 3. Parser::parse(tokens) → AST
  // 4. TimeNorm pass on AST
  // 5. InheritExpander::expand
  // 6. Resolver::register_document + resolve_all
  // 7. atom_out.emit / hami_out.emit → write to out_dir
  //
  // Scaffold: log only.
  eprintln!("[compile] {}", file.display());
  Ok(())
}
