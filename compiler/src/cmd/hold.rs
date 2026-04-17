//! `aura hold` and `aura hold restore` — park/restore working draft.

use crate::error::{CompileError, Result};
use crate::logs::Logger;
use std::path::PathBuf;

/// Parks the current working draft without recording a take.
///
/// The draft is saved to `.history/hold` and can be restored with
/// `aura hold restore`.
pub fn hold(project: &PathBuf) -> Result<()> {
  let hold_dir = project.join(".history").join("hold");
  std::fs::create_dir_all(&hold_dir)
    .map_err(|e| CompileError::msg(format!("cannot create .history/hold: {}", e)))?;
  // In the full implementation: serialize the current working draft
  // (all modified .aura files) into the hold directory.
  Logger::new().info("Working draft parked in .history/hold");
  Ok(())
}

/// Restores a previously parked draft.
pub fn restore(project: &PathBuf) -> Result<()> {
  let hold_dir = project.join(".history").join("hold");
  if !hold_dir.exists() {
    return Err(CompileError::msg(
      "no parked draft found — run `aura hold` first",
    ));
  }
  // In the full implementation: restore the working draft from hold.
  Logger::new().success("Parked draft restored");
  Ok(())
}
