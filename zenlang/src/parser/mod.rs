//! Parser
//!
//! Contains a parser for generating AST
mod block;
pub mod error;
mod expression;
mod func;
mod if_chain;
mod parser;
mod statement;
mod r#while;
pub use parser::*;
