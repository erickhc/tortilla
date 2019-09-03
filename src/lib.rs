//! # Tortilla
//! Tortilla is a wrapper over the solc compiler.
//!
//! It is provided both as a binary and a library.

//! Example:
//! ```rust
//! use tortilla::compiler::compile_path;
//!
//! let contracts = compile_path("tests/contracts").unwrap();
//!
//! for contract in contracts.iter() {
//!     println!("{}", contract.pretty_print());
//! }
//! ```

pub mod abi;

/// Functions to call _solc_ over arbitrary path(s)
pub mod compiler;

/// Contracts parsed from _solc_ output
pub mod contract;
mod solc;
