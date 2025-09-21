use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Call(),
    Vmcall(u8),
    Loadcn(f64), // load contant number
    Loadcnu(),
    Loadcs(String), // load constant string
    Loadv(String),  // load variable
    Storev(String), // store variable
    Pushret(),      // push the ret register
    Cafse(u64),     // construct array from stack elements
    Iafs(),         // Index array from stack
    Aiafs(String),
    Bfas(), // Begin function arguments setup
    Efas(), // End function arguments setup
    Pop(),  // pop from stack
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
