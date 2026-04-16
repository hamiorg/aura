//! Config module — toolchain configuration reader.
//!
//! Reads the `configs/` folder (never compiled, never history-tracked).
//!
//! # Sub-modules
//!
//! | Module   | Responsibility                              |
//! | -------- | ------------------------------------------- |
//! | `load`   | Reads `configs/llm.aura` and `stores.aura`  |
//! | `ignore` | Reads `configs/ignore.aura` exclusion list  |
//!
//! # Module access paths
//!
//! ```
//! compiler::cfg::load::ConfigLoader
//! compiler::cfg::load::Config
//! compiler::cfg::load::LlmProvider
//! compiler::cfg::load::StoreDecl
//! compiler::cfg::ignore::IgnoreList
//! compiler::cfg::ignore::BUILTIN
//! ```

pub mod ignore;
pub mod load;

pub use ignore::IgnoreList;
pub use load::{Config, ConfigLoader, LlmProvider, StoreDecl};
