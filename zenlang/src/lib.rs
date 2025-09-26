//! ZenLang
//!
//! Primary programming language of ZenOS
//! ```ignore
//! fn main {
//!     println("Hello from ZenLang!");
//!     return null;
//! }
//! ```
#![no_std]
pub mod ast;
pub mod compiler;
pub mod module;
pub mod opcode;
pub mod parser;
pub mod platform;
pub mod scope;
pub mod stdlib;
pub mod strong_u64;
pub mod tokenizer;
pub mod unescape;
pub mod value;
pub mod vm;

mod func_attr;
pub use func_attr::*;

extern crate alloc;
