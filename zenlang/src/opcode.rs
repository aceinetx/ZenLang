use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Call(),
    Vmcall(u8),
    Loadcn(f64),    // load contant number
    Loadcnu(),      // load constant null
    Loadcb(bool),   // load constant boolean
    Loadcs(String), // load constant string
    Loadv(String),  // load variable
    Storev(String), // store variable
    Pushret(),      // push the ret register
    Cafse(u64),     // construct array from stack elements
    Iafs(),         // Index array from stack
    Aiafs(String),
    Bfas(),   // Begin function arguments setup
    Efas(),   // End function arguments setup
    Pop(),    // pop from stack
    Bst(u32), // branch stack true (branch if stack value is true)
    Br(u32),  // branch
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
    Ret(),
}
