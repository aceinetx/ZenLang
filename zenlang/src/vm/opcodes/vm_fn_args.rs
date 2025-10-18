use crate::vm::VM;
impl VM {
    pub fn op_begin_fn_args(&mut self) {
        self.bfas_stack_start.push(self.stack.len() as i64);
    }

    pub fn op_end_fn_args(&mut self) {
        self.bfas_stack_end.push(self.stack.len() as i64);
    }
}
