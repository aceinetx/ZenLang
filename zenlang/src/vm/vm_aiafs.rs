use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::string::String;

impl VM {
    /// Perform Aiafs operation
    ///
    /// The reason it's in a function (not in execute_opcode) is because it needs to be recursive
    pub(crate) fn aiafs(&mut self, value: &mut Value, set_to: Value, index: Value) {
        match value {
            Value::Object(obj) => match &mut *obj.borrow_mut() {
                Object::Array(array) => {
                    let usz_index: usize;
                    if let Value::Number(index) = index {
                        usz_index = index as usize;
                    } else {
                        self.error =
                            format!("aiafs failed: expected number when indexing an array");
                        return;
                    }

                    if usz_index > array.len() {
                        self.error = format!("aiafs failed: index outside bounds");
                        return;
                    } else if usz_index == array.len() {
                        array.push(set_to);
                        return;
                    }

                    array[usz_index] = set_to;
                }
                Object::Dictionary(dict) => {
                    let s_index: String;
                    if let Value::String(s) = index {
                        s_index = s;
                    } else {
                        self.error =
                            format!("aiafs failed: expected string when indexing a dictionary");
                        return;
                    }

                    dict.insert(s_index, set_to);
                }
            },
            _ => {
                self.error = format!("aiafs failed: value is not an object");
            }
        }
        return;
    }
}
