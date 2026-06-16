use crate::value::*;
use crate::vm::VM;
use crate::vm::VMError;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_iafs(&mut self) -> Result<(), VMError> {
        let index = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("iafs failed: no more values on stack for index".into());
            }
        };
        let array = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("iafs failed: no more values on stack for array".into());
            }
        };

        self.self_var = array.clone();

        match &array {
            Value::Object(obj) => match &*obj.borrow() {
                Object::Array(array) => {
                    if let Value::Number(index) = index {
                        let usize_index = index as usize;
                        if usize_index >= array.len() {
                            self.stack.push(Value::Null());
                            return Ok(());
                        }

                        self.stack.push(array[usize_index].clone());
                        return Ok(());
                    }
                }
                Object::Dictionary(dict) => {
                    if let Value::String(index) = index {
                        if !dict.contains_key(&index) {
                            self.stack.push(Value::Null());
                        } else {
                            self.stack.push(dict.get(&index).unwrap().clone());
                        }
                        return Ok(());
                    }
                }
            },
            Value::String(string) => {
                if let Value::Number(index) = index {
                    if let Some(ch) = string.chars().nth(index as usize) {
                        self.stack.push(Value::String(String::from(ch)));
                    } else {
                        self.stack.push(Value::Null());
                    }
                    return Ok(());
                }
            }
            _ => {}
        }

        return Err(format!(
            "iafs failed: invalid operand types: {:?} {:?}",
            array.get_type(),
            index.get_type()
        ));
    }
}
