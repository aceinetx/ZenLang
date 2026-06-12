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
    Dynvmcall(),
    LoadConstant(f64),  // load contant number
    LoadNull(),         // load constant null
    LoadBool(bool),     // load constant boolean
    LoadStr(String),    // load constant string
    LoadVar(String),    // load variable
    StoreVar(String),   // store variable
    PushRet(),          // push the ret register
    Cafse(u64),         // construct array from stack elements
    Iafs(),             // Index array or dictionary from stack
    Cdfse(Vec<String>), // construct dictionary from stack elements
    Aiafs(),
    BeginArgs(),
    PushArg(),
    StoreArg(String),
    Pop(),                // pop from stack
    BranchTrue(usize),    // branch stack true (branch if stack value is true)
    BranchNonNull(usize), // branch stack non null
    Branch(usize),        // branch
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
    Lambda(usize, usize),
}
