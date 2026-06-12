use crate::value::*;
use crate::vm::VM;
use alloc::format;

impl VM {
    pub fn op_branch_true(&mut self, addr: usize) {
        if let Some(value) = self.stack.pop() {
            if let Value::Boolean(flag) = value {
                if flag {
                    self.pc.inst = addr - 1;
                }
                return;
            }
            if let Value::Number(num) = value {
                if num != 0.0 {
                    self.pc.inst = addr - 1;
                }
                return;
            }

            self.error = format!(
                "bst failed: value is not of an acceptable type ({:?})",
                value
            );
        } else {
            self.error = "bst failed: no value on stack".into();
        }
    }

    pub fn op_branch_nonnull(&mut self, addr: usize) {
        if let Some(value) = self.stack.pop() {
            if let Value::Null() = value {
                return;
            }
            self.pc.inst = addr - 1;
        } else {
            self.error = "bsnn failed: no value on stack".into();
        }
    }

    pub fn op_branch(&mut self, addr: usize) {
        self.pc.inst = addr - 1;
    }
}
