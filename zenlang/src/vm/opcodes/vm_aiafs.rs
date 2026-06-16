use crate::value::*;
use crate::vm::VM;
use crate::vm::VMError;
use alloc::string::*;

impl VM {
    /// Perform Aiafs operation
    ///
    /// The reason it's in a function (not in execute_opcode) is because it needs to be recursive
    fn aiafs(&mut self, value: &mut Value, set_to: Value, index: Value) -> Result<(), VMError> {
        match value {
            Value::Object(obj) => match &mut *obj.borrow_mut() {
                Object::Array(array) => {
                    let usz_index: usize;
                    if let Value::Number(index) = index {
                        usz_index = index as usize;
                    } else {
                        return Err("aiafs failed: expected number when indexing an array".into());
                    }

                    if usz_index > array.len() {
                        return Err("aiafs failed: index outside bounds".into());
                    } else if usz_index == array.len() {
                        array.push(set_to);
                        return Ok(());
                    }

                    array[usz_index] = set_to;
                }
                Object::Dictionary(dict) => {
                    let s_index: String;
                    if let Value::String(s) = index {
                        s_index = s;
                    } else {
                        return Err(
                            "aiafs failed: expected string when indexing a dictionary".into()
                        );
                    }

                    dict.insert(s_index, set_to);
                }
            },
            _ => {
                return Err("aiafs failed: value is not an object".into());
            }
        }
        return Ok(());
    }

    pub fn op_aiafs(&mut self) -> Result<(), VMError> {
        let index = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("aiafs failed: no more values on stack for index".into());
            }
        };
        let set_value = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("aiafs failed: no more values on stack for set value".into());
            }
        };
        let mut object = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("aiafs failed: no more values on stack for object".into());
            }
        };

        return self.aiafs(&mut object, set_value, index);
    }
}
