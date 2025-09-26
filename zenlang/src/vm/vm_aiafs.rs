use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::vec::*;

impl VM {
    /// Perform Aiafs operation
    ///
    /// The reason it's in a function (not in execute_opcode) is because it needs to be recursive
    pub(crate) fn aiafs(&mut self, value: Value, set_to: Value, mut indexes: Vec<Value>) -> Value {
        match value {
            Value::Array(mut array) => {
                let index = indexes[0].clone();
                if let Value::Number(index) = index {
                    let usize_index = index as usize;
                    if usize_index == array.len() {
                        array.push(set_to);
                        return Value::Array(array);
                    } else if usize_index >= array.len() {
                        self.error = format!(
                            "aiafs failed: index ({}) is larger or equal to array length ({})",
                            usize_index,
                            array.len()
                        );
                        return Value::Null();
                    }

                    if indexes.len() == 1 {
                        array[usize_index] = set_to;
                        return Value::Array(array);
                    } else {
                        indexes.remove(0);
                        if let Value::Array(inner) = &array[usize_index] {
                            array[usize_index] =
                                self.aiafs(Value::Array(inner.clone()), set_to, indexes);
                            return Value::Array(array);
                        }
                    }
                }
            }
            Value::Dictionary(mut dict) => {
                let index = indexes[0].clone();
                if let Value::String(index) = index {
                    for element in dict.iter_mut() {
                        if element.0 != index {
                            continue;
                        }

                        if indexes.len() == 1 {
                            (*element).1 = set_to;
                            return Value::Dictionary(dict);
                        } else {
                            indexes.remove(0);
                            if let Value::Dictionary(inner) = &element.1 {
                                (*element).1 =
                                    self.aiafs(Value::Dictionary(inner.clone()), set_to, indexes);
                                return Value::Dictionary(dict);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        self.error = format!("aiafs failed: invalid operand types");
        return Value::Null();
    }
}
