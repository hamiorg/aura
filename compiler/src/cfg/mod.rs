//! Config module — toolchain configuration reader.
//!
//! Reads the `configs/` folder (never compiled, never history-tracked).
//!
//! # Sub-modules
//!
//! | Module   | Responsibility                              |
//! | -------- | ------------------------------------------- |
//! | `load`        | Reads `configs/llm.aura` and `stores.aura`           |
//! | `ignore`      | Reads `configs/ignore.aura` exclusion list           |
//! | `metaboolean` | Reads `meta/metaboolean.aura` custom boolean map     |
//! | `metaaccess`  | Reads `meta/metaaccess.aura` ReBAC DAG + topo-sort   |
//!
//! # Module access paths
//!
//! ```ignore
//! compiler::cfg::load::ConfigLoader
//! compiler::cfg::load::Config
//! compiler::cfg::load::LlmProvider
//! compiler::cfg::load::StoreDecl
//! compiler::cfg::ignore::IgnoreList
//! compiler::cfg::ignore::BUILTIN
//! compiler::cfg::metaboolean::BooleanMap
//! compiler::cfg::metaaccess::AccessWeights
//! ```

pub mod ignore;
pub mod load;
pub mod metaaccess;
pub mod metaboolean;

pub use ignore::IgnoreList;
pub use load::{Config, ConfigLoader, LlmProvider, StoreDecl};
pub use metaaccess::AccessWeights;
pub use metaboolean::BooleanMap;
