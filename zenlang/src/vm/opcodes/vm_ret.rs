use crate::vm::VM;

impl VM {
    pub fn op_push_ret(&mut self) {
        self.stack.push(self.ret.clone());
    }

    pub fn op_ret(&mut self) {
        if !self.stack.is_empty() {
            self.ret = self.stack.pop().unwrap();
        }

        self.remove_environment();

        if !self.call_stack.is_empty() {
            self.pc = self.call_stack.pop().unwrap();
        } else {
            self.halted = true;
        }
    }
}
