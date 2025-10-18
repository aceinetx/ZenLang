//! VM
//!
//! Contains the virtual machine for ZenLang
pub mod opcodes;
mod vm;
mod vm_compute;
mod vm_opcode;
mod vmcall;
pub use vm::*;
