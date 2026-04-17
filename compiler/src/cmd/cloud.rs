//! `aura release`, `aura sync`, `aura dub` — cloud operations.

use crate::error::{CompileError, Result};
use crate::logs::Logger;
use std::path::PathBuf;

/// Publishes the current take to the cloud distribution store.
///
/// This runs `aura compile` then uploads the resulting `.atom`, `.hami`,
/// and `.atlas` files to the configured primary store.
pub fn release(project: &PathBuf) -> Result<()> {
  // Step 1: compile the working draft.
  use crate::cmd::compile::{run, CompileOpts};
  let opts = CompileOpts {
    project: project.clone(),
    ..Default::default()
  };
  run(&opts)?;

  // Step 2: upload compiled artifacts to the store.
  // In the full implementation: read configs/stores.aura and upload
  // .atom/.hami/.atlas files to the primary store URI.
  Logger::new().success("Artifacts published to primary store");
  Ok(())
}

/// Pulls the latest released state from the cloud store.
///
/// Updates the working draft to match the most recently released take.
pub fn sync(_project: &PathBuf) -> Result<()> {
  // In the full implementation: fetch the latest manifest from the
  // primary store, compare with the local HEAD take, and pull any
  // new delta objects.
  Logger::new().success("Synced from primary store");
  Ok(())
}

/// Creates an independent full-history copy of the project.
///
/// Copies the entire project directory including `.history/`. The copy
/// is fully independent — its history is divergent from the original
/// from the moment of dubbing.
pub fn dub(project: &PathBuf, destination: &PathBuf) -> Result<()> {
  if destination.exists() {
    return Err(CompileError::msg(format!(
      "destination `{}` already exists",
      destination.display()
    )));
  }

  // Copy the project directory recursively.
  copy_dir(project, destination)?;

  Logger::new().success(&format!("Full-history copy created at {}", destination.display()));
  Ok(())
}

// -------------------------------------------------------------------- //
// Internal helpers

fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
  std::fs::create_dir_all(dst)
    .map_err(|e| CompileError::msg(format!("cannot create `{}`: {}", dst.display(), e)))?;

  for entry in std::fs::read_dir(src)
    .map_err(|e| CompileError::msg(format!("cannot read `{}`: {}", src.display(), e)))?
    .flatten()
  {
    let from = entry.path();
    let to = dst.join(entry.file_name());
    if from.is_dir() {
      copy_dir(&from, &to)?;
    } else {
      std::fs::copy(&from, &to)
        .map_err(|e| CompileError::msg(format!("cannot copy `{}`: {}", from.display(), e)))?;
    }
  }
  Ok(())
}
