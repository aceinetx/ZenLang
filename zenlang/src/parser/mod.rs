//! Parser
//!
//! Contains a parser for generating AST
pub mod error;
mod expression;
mod func;
mod parser;
mod statement;
pub use parser::*;
