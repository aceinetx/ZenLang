use crate::vm::VMError;

pub enum StopReason {
    Halt,
    Breakpoint,
}
