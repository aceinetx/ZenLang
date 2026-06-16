use crate::vm::*;

impl VM {
    pub fn op_push_ret(&mut self) -> Result<(), VMError> {
        self.stack.push(self.ret.clone());
        return self.check_stack_overflow();
    }

    pub fn op_ret(&mut self) -> Result<(), VMError> {
        self.ret = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("ret failed: stack is empty".into());
            }
        };

        self.remove_scope();

        if !self.call_stack.is_empty() {
            self.pc = self.call_stack.pop().unwrap();
        } else {
            return Err("ret failed: call stack is empty".into());
        }

        return Ok(());
    }
}
