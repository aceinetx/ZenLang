//! Parser
//!
//! Contains a parser for generating AST
mod parser;
mod parser_block;
mod parser_error;
mod parser_expr;
mod parser_func;
mod parser_if_chain;
mod parser_prec;
mod parser_stmt;
pub use parser::*;
