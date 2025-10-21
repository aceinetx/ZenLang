use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_iafs(&mut self) {
        let array;
        let index;
        if let Some(value) = self.stack.pop() {
            index = value;
        } else {
            self.error = format!("iafs failed: no more values on stack for index");
            return;
        }
        if let Some(value) = self.stack.pop() {
            array = value;
        } else {
            self.error = format!("iafs failed: no more values on stack for array");
            return;
        }
        self.this = array.clone();

        match array {
            Value::Object(obj) => match &*obj.borrow() {
                Object::Array(array) => {
                    if let Value::Number(index) = index {
                        let usize_index = index as usize;
                        if usize_index >= array.len() {
                            self.stack.push(Value::Null());
                            return;
                        }

                        self.stack.push(array[usize_index].clone());
                        return;
                    }
                }
                Object::Dictionary(dict) => {
                    if let Value::String(index) = index {
                        if !dict.contains_key(&index) {
                            self.stack.push(Value::Null());
                        } else {
                            self.stack.push(dict.get(&index).unwrap().clone());
                        }
                        return;
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
                    return;
                }
            }
            _ => {
                self.error = format!(
                    "iafs failed: invalid operand types: {:?} {:?}",
                    array, index
                );
            }
        }
    }
}
