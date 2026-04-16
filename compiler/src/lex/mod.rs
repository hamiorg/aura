//! Lexer module — zero-copy tokenizer for AURA source files.
//!
//! # Usage
//!
//! ```rust,no_run
//! use compiler::lex::scan::Scanner;
//! use compiler::lex::token::Kind;
//!
//! let src = r#"manifest::
//!   name -> "Signal Loss"
//! "#;
//!
//! let scanner = Scanner::new(src);
//! let tokens = scanner.collect_all().unwrap();
//! for tok in &tokens {
//!     println!("{:?}", tok.kind);
//! }
//! ```
//!
//! # Design
//!
//! The lexer never allocates heap memory. All string slices in tokens
//! are `&'src str` tied to the source buffer's lifetime. The hot path
//! is a single branch `if byte < 0x20 → structural token` which enables
//! AVX-2 SIMD vectorization at 32 bytes per CPU clock cycle.

pub mod scan;
pub mod token;

pub use token::{Kind, Token};
