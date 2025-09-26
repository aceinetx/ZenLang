use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

impl VM {
    /// Performs a vmcall
    ///
    /// ### VMCall indexes
    /// - 1: print
    /// - 2: println
    /// - 3: get_string
    pub fn vmcall(&mut self, index: u8) {
        match index {
            1 => {
                // print
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.print(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            2 => {
                // println
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.println(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            3 => {
                // get_string
                if let Some(platform) = &self.platform {
                    let string = platform.get_string();
                    let value = Value::String(string.into());
                    self.stack.push(value);
                }
            }
            4 => {
                // load module dynamically
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        if let Value::String(name) = value {
                            //platform.println(format!("{}", value));
                            self.error = format!("module not found: {}", name);
                            if let Some(module) = platform.get_module(name) {
                                self.error.clear();

                                let _ = self.load_module(&module);
                            }
                            return;
                        } else {
                            self.error = "vmcall: expected a string".into();
                        }
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            5 => {
                // array size
                if let Some(value) = self.stack.pop() {
                    if let Value::Array(array) = value {
                        self.stack.push(Value::Number(array.len() as f64));
                    } else {
                        self.error = "vmcall: expected an array".into();
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                }
            }
            6 => {
                // array push
                let mut array;
                let element;

                if let Some(value) = self.stack.pop() {
                    element = value;
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::Array(value) = value {
                        array = value;
                    } else {
                        self.error = "vmcall: expected an array".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                array.push(element);
                self.stack.push(Value::Array(array));
            }
            7 => {
                // array pop
                let mut array;

                if let Some(value) = self.stack.pop() {
                    if let Value::Array(value) = value {
                        array = value;
                    } else {
                        self.error = "vmcall: expected an array".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                array.pop();
                self.stack.push(Value::Array(array));
            }
            8 => {
                // array remove
                let mut array;
                let at;

                if let Some(value) = self.stack.pop() {
                    if let Value::Number(value) = value {
                        at = value as usize;
                    } else {
                        self.error = "vmcall: expected a number".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::Array(value) = value {
                        array = value;
                    } else {
                        self.error = "vmcall: expected an array".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if at >= array.len() {
                    self.stack.push(Value::Null());
                    return;
                }
                array.remove(at);
                self.stack.push(Value::Array(array));
            }
            9 => {
                // array insert
                let mut array;
                let at;
                let element;

                if let Some(value) = self.stack.pop() {
                    element = value;
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::Number(value) = value {
                        at = value as usize;
                    } else {
                        self.error = "vmcall: expected a number".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::Array(value) = value {
                        array = value;
                    } else {
                        self.error = "vmcall: expected an array".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if at >= array.len() {
                    self.stack.push(Value::Null());
                    return;
                }
                array.insert(at, element);
                self.stack.push(Value::Array(array));
            }
            10 => {
                // string split
                let string;
                let delimiter;

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        delimiter = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        string = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                let mut array: Vec<Value> = Vec::new();
                for part in string.split(&delimiter) {
                    array.push(Value::String(String::from(part)))
                }
                self.stack.push(Value::Array(array));
            }
            _ => {
                self.error = format!("vmcall: invalid vmcall index {}", index);
            }
        }
    }
}
