use crate::value::*;
use crate::vm::VM;
use alloc::string::*;

impl VM {
    pub fn op_load_constant(&mut self, value: f64) {
        let value = Value::Number(value);
        self.stack.push(value);
        self.check_stack_overflow();
    }

    pub fn op_load_null(&mut self) {
        self.stack.push(Value::Null());
        self.check_stack_overflow();
    }

    pub fn op_load_bool(&mut self, flag: bool) {
        self.stack.push(Value::Boolean(flag));
        self.check_stack_overflow();
    }

    pub fn op_load_str(&mut self, value: &String) {
        let value = Value::String(value.to_string());
        self.stack.push(value);
        self.check_stack_overflow();
    }
}
