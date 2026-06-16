use crate::value::*;
use crate::vm::{VM, VMError};
use alloc::string::*;

impl VM {
    pub fn op_load_constant(&mut self, value: f64) -> Result<(), VMError> {
        let value = Value::Number(value);
        self.stack.push(value);
        self.check_stack_overflow()
    }

    pub fn op_load_null(&mut self) -> Result<(), VMError> {
        self.stack.push(Value::Null());
        self.check_stack_overflow()
    }

    pub fn op_load_bool(&mut self, flag: bool) -> Result<(), VMError> {
        self.stack.push(Value::Boolean(flag));
        self.check_stack_overflow()
    }

    pub fn op_load_str(&mut self, value: &String) -> Result<(), VMError> {
        let value = Value::String(value.to_string());
        self.stack.push(value);
        self.check_stack_overflow()
    }
}
