use crate::value::*;
use crate::vm::*;
use alloc::format;

impl VM {
    pub fn op_branch_true(&mut self, addr: usize) -> Result<(), VMError> {
        let value = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("bst failed: no value on stack".into());
            }
        };

        return match value {
            Value::Boolean(flag) => {
                if flag {
                    self.pc.inst = addr - 1;
                }
                Ok(())
            }
            Value::Number(num) => {
                if num != 0.0 {
                    self.pc.inst = addr - 1;
                }
                Ok(())
            }
            _ => Err(format!(
                "bst failed: value is not of an acceptable type ({:?})",
                value
            )),
        };
    }

    pub fn op_branch_nonnull(&mut self, addr: usize) -> Result<(), VMError> {
        let value = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("bsnn failed: no value on stack".into());
            }
        };

        if let Value::Null() = value {
            return Ok(());
        }

        self.pc.inst = addr - 1;

        Ok(())
    }

    pub fn op_branch(&mut self, addr: usize) -> Result<(), VMError> {
        self.pc.inst = addr - 1;
        Ok(())
    }
}
