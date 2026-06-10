//! Parser
//!
//! Contains a parser for generating AST
pub mod error;
mod expression;
mod func;
#[macro_use]
mod parser;
mod statement;
pub use parser::*;
