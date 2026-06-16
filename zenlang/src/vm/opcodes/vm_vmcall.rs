use crate::value::*;
use crate::vm::*;

impl VM {
    pub fn op_vmcall(&mut self, index: u8) -> Result<(), VMError> {
        match self.args.pop() {
            Some(mut args) => {
                while !args.is_empty() {
                    self.stack.push(args.remove(0));
                }
            }
            None => (),
        }

        self.vmcall(index);

        return Ok(());
    }

    pub fn op_dynvmcall(&mut self) -> Result<(), VMError> {
        match self.args.pop() {
            Some(mut args) => {
                while !args.is_empty() {
                    self.stack.push(args.remove(0));
                }
            }
            None => (),
        }

        let index = match self.stack.pop() {
            Some(value) => match value {
                Value::Number(value) => value as i64 as u8,
                _ => {
                    return Err("dynvmcall failed: value on stack is not a number".into());
                }
            },
            None => {
                return Err("dynvmcall failed: no more values on stack".into());
            }
        };

        self.vmcall(index);

        return Ok(());
    }
}
