//! CLI command handlers module.
//!
//! Each sub-module handles one group of `aura` CLI commands.
//!
//! # Command → module mapping
//!
//! | Command group               | Module    |
//! | --------------------------- | --------- |
//! | `aura compile/validate/lint`| `compile` |
//! | `aura validate`, `lint`     | `check`   |
//! | `aura generate`             | `gen`     |
//! | `aura init`, `add`          | `init`    |
//! | `aura take/mark/rewind/…`   | `take`    |
//! | `aura stream/mix`           | `stream`  |
//! | `aura hold`                 | `hold`    |
//! | `aura release/sync/dub`     | `cloud`   |

pub mod check;
pub mod cloud;
pub mod compile;
pub mod gen;
pub mod hold;
pub mod init;
pub mod stream;
pub mod take;
