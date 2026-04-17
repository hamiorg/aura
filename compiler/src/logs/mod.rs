//! AURA compiler logging system.
//!
//! Zero-dependency, coloured terminal output for the `aura` toolchain CLI.
//! All output goes to **stderr** to keep stdout clean for machine-readable
//! output in future piped workflows.
//!
//! # Usage
//!
//! ```rust,ignore
//! use compiler::logs::Logger;
//!
//! let log = Logger::new();
//!
//! // Phase messages
//! log.compile("Building album project…");
//! log.parse("Parsing info/credits.aura");
//! log.lint("Checking 3 files");
//! log.emit("Writing dist/album.atom");
//! log.success("Compilation complete — 14 files");
//!
//! // Diagnostics with file + line context
//! log.warn("meta/moods.aura", 1, Some("W003"), "node `moods` has no `time` field", None);
//! log.error(
//!   "info/credits.aura", 5, None,
//!   "expected `->` at line 5, got Comma",
//!   Some("Add `->` between the key and value"),
//! );
//! ```
//!
//! # Log kinds and their colours
//!
//! | Kind      | Colour           | When to use                                  |
//! | --------- | ---------------- | -------------------------------------------- |
//! | `compile` | bold cyan        | Top-level compile phase messages             |
//! | `lex`     | dim cyan         | Lexer progress (verbose only)                |
//! | `parse`   | cyan             | Parser file-by-file progress                 |
//! | `lint`    | bright yellow    | Linter rule pass progress                    |
//! | `emit`    | bright blue      | Emitter output messages                      |
//! | `info`    | blue             | General information                          |
//! | `debug`   | gray             | Debug / verbose output                       |
//! | `note`    | magenta          | Supplementary notes attached to diagnostics  |
//! | `warn`    | bold yellow      | Non-fatal warnings (W001–W006)               |
//! | `error`   | bold red         | Fatal compile errors (E001, parse errors)    |
//! | `success` | bold green       | Successful completion                        |

pub mod colors;
pub mod formatter;
pub mod logger;

pub use logger::Logger;
