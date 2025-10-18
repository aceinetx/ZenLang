use crate::value::*;
use crate::vm::VM;
use alloc::format;

impl VM {
    pub fn op_vmcall(&mut self, index: u8) {
        self.vmcall(index);
    }

    pub fn op_dynvmcall(&mut self) {
        let index;
        if let Some(value) = self.stack.pop() {
            if let Value::Number(value) = value {
                index = value as i64 as u8;
            } else {
                self.error = format!("dynvmcall failed: value on stack is not a number");
                return;
            }
        } else {
            self.error = format!("dynvmcall failed: no more values on stack");
            return;
        }
        self.vmcall(index);
    }
}
