//! Toolchain config reader — reads the `configs/` folder.
//!
//! The `configs/` folder is **never compiled** and **never tracked** by
//! `.history/`. It holds toolchain configuration only:
//!
//! ```text
//! configs/
//!   llm.aura       <- LLM provider definitions (editor integration)
//!   stores.aura    <- remote store origins and authentication refs
//!   account.aura   <- cloud identity (reads env vars, no raw secrets)
//!   ignore.aura    <- extra paths excluded from .history/ tracking
//! ```
//!
//! Credential values are never stored in `configs/account.aura`. That
//! file declares environment variable names. Actual secrets come from
//! `.env` or the process environment.

use crate::error::{CompileError, Result};
use std::collections::HashMap;
use std::path::PathBuf;

/// LLM provider configuration from `configs/llm.aura`.
#[derive(Debug, Clone)]
pub struct LlmProvider {
  /// Provider name, e.g. `"openai"`, `"anthropic"`, `"ollama"`.
  pub name: String,
  /// Model identifier, e.g. `"gpt-4o"`, `"claude-3-5-sonnet"`.
  pub model: String,
  /// Optional local endpoint URL (for ollama / self-hosted).
  pub endpoint: Option<String>,
  /// Environment variable name holding the API key.
  pub auth_env: Option<String>,
}

/// A store backend declaration from `configs/stores.aura`.
#[derive(Debug, Clone)]
pub struct StoreDecl {
  /// Alias, e.g. `"primary"`, `"backup"`, `"local"`.
  pub alias: String,
  /// Store URI, e.g. `"aura://store.hami.aduki.org/catalogs/cx0ab3de"`.
  pub uri: String,
  /// Store kind: `"aduki"`, `"cloudflare-r2"`, `"s3"`, `"filesystem"`.
  pub kind: String,
  /// `@account/id` reference for credential lookup.
  pub auth: Option<String>,
}

/// Parsed contents of the `configs/` folder.
#[derive(Debug, Clone, Default)]
pub struct Config {
  /// All LLM providers from `configs/llm.aura`.
  pub llm: Vec<LlmProvider>,
  /// All store declarations from `configs/stores.aura`.
  pub stores: Vec<StoreDecl>,
}

impl Config {
  /// Returns the primary store declaration, if any.
  pub fn primary_store(&self) -> Option<&StoreDecl> {
    self.stores.iter().find(|s| s.alias == "primary")
  }

  /// Returns the local/filesystem store, if any.
  pub fn local_store(&self) -> Option<&StoreDecl> {
    self.stores.iter().find(|s| s.alias == "local")
  }
}

/// Toolchain config loader.
pub struct ConfigLoader {
  /// Project root directory.
  root: PathBuf,
}

impl ConfigLoader {
  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self { root: root.into() }
  }

  /// Reads the `configs/` folder and returns the parsed configuration.
  /// Missing files are silently skipped (configs are optional).
  pub fn load(&self) -> Result<Config> {
    let mut cfg = Config::default();

    let llm_path = self.root.join("configs").join("llm.aura");
    let stores_path = self.root.join("configs").join("stores.aura");

    if llm_path.exists() {
      let text = std::fs::read_to_string(&llm_path)
        .map_err(|e| CompileError::msg(format!("cannot read configs/llm.aura: {}", e)))?;
      cfg.llm = parse_llm(&text);
    }

    if stores_path.exists() {
      let text = std::fs::read_to_string(&stores_path)
        .map_err(|e| CompileError::msg(format!("cannot read configs/stores.aura: {}", e)))?;
      cfg.stores = parse_stores(&text);
    }

    Ok(cfg)
  }
}

// -------------------------------------------------------------------- //
// Minimal parsers (line-by-line for config files)

fn parse_llm(text: &str) -> Vec<LlmProvider> {
  let mut providers = Vec::new();
  let mut current: Option<HashMap<String, String>> = None;
  let mut current_name = String::new();

  for line in text.lines() {
    let trimmed = line.trim();
    if trimmed.ends_with("::") && !trimmed.starts_with(' ') {
      if let Some(prev) = current.take() {
        if let Some(p) = build_llm(&current_name, &prev) {
          providers.push(p);
        }
      }
      current_name = trimmed.trim_end_matches("::").to_string();
      current = Some(HashMap::new());
      continue;
    }
    if let Some(ref mut map) = current {
      if let Some((k, v)) = parse_kv(trimmed) {
        map.insert(k.to_string(), v.to_string());
      }
    }
  }
  if let Some(prev) = current.take() {
    if let Some(p) = build_llm(&current_name, &prev) {
      providers.push(p);
    }
  }
  providers
}

fn build_llm(name: &str, map: &HashMap<String, String>) -> Option<LlmProvider> {
  if name == "llm" {
    return None;
  } // top-level block
  Some(LlmProvider {
    name: name.to_string(),
    model: map.get("model").cloned().unwrap_or_default(),
    endpoint: map.get("endpoint").cloned(),
    auth_env: map.get("env").cloned(),
  })
}

fn parse_stores(text: &str) -> Vec<StoreDecl> {
  let mut stores = Vec::new();
  let mut current: Option<HashMap<String, String>> = None;
  let mut current_name = String::new();

  for line in text.lines() {
    let trimmed = line.trim();
    if trimmed.ends_with("::") && !trimmed.starts_with(' ') {
      if let Some(prev) = current.take() {
        if let Some(s) = build_store(&current_name, &prev) {
          stores.push(s);
        }
      }
      current_name = trimmed.trim_end_matches("::").to_string();
      current = Some(HashMap::new());
      continue;
    }
    if let Some(ref mut map) = current {
      if let Some((k, v)) = parse_kv(trimmed) {
        map.insert(k.to_string(), v.to_string());
      }
    }
  }
  if let Some(prev) = current.take() {
    if let Some(s) = build_store(&current_name, &prev) {
      stores.push(s);
    }
  }
  stores
}

fn build_store(alias: &str, map: &HashMap<String, String>) -> Option<StoreDecl> {
  if alias == "stores" {
    return None;
  }
  Some(StoreDecl {
    alias: alias.to_string(),
    uri: map.get("uri").cloned().unwrap_or_default(),
    kind: map.get("kind").cloned().unwrap_or_default(),
    auth: map.get("auth").cloned(),
  })
}

fn parse_kv(line: &str) -> Option<(&str, &str)> {
  let arrow = line.find("->")?;
  let key = line[..arrow].trim();
  let val = line[arrow + 2..].trim().trim_matches('"');
  if key.is_empty() {
    return None;
  }
  Some((key, val))
}
