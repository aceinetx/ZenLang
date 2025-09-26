//! Opcode
//!
//! What the virtual machine executes
use alloc::string::*;
use alloc::vec::*;
use bincode::*;

/// Opcode
#[derive(Encode, Decode, Debug, Clone)]
pub enum Opcode {
    Call(),
    Vmcall(u8),
    Loadcn(f64),        // load contant number
    Loadcnu(),          // load constant null
    Loadcb(bool),       // load constant boolean
    Loadcs(String),     // load constant string
    Loadv(String),      // load variable
    Storev(String),     // store variable
    Pushret(),          // push the ret register
    Cafse(u64),         // construct array from stack elements
    Iafs(),             // Index array or dictionary from stack
    Cdfse(Vec<String>), // construct dictionary from stack elements
    Aiafs(String, u64),
    Bfas(),    // Begin function arguments setup
    Efas(),    // End function arguments setup
    Pop(),     // pop from stack
    Bst(u32),  // branch stack true (branch if stack value is true)
    Bsnn(u32), // branch stack non null
    Br(u32),   // branch
    Add(),
    Sub(),
    Mul(),
    Div(),
    Eq(),
    Neq(),
    Lt(),
    Gt(),
    Le(),
    Ge(),
    Bshr(),
    Bshl(),
    Band(),
    Bor(),
    Ret(),
}
