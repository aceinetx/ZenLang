//! VM
//!
//! Contains the virtual machine for ZenLang
pub mod opcodes;
mod program_counter;
mod stop_reason;
mod vm;
mod vm_compute;
mod vm_opcode;
mod vmcall;
pub use program_counter::*;
pub use stop_reason::*;
pub use vm::*;
