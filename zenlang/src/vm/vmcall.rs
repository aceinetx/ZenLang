use crate::interop::*;
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
        // DISABLED FOR NOW
        /*
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
                    if let Value::Object(obj) = value {
                        unsafe {
                            if let Object::Array(array) = obj.read() {
                                self.stack.push(Value::Number(array.len() as f64));
                            } else {
                                self.error = "vmcall: expected an array".into();
                            }
                        }
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
            11 => {
                // read file bytes
                let name;

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        name = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(platform) = &self.platform {
                    if let Some(bytes) = platform.read_file_bytes(name) {
                        let mut array: Vec<Value> = Vec::new();
                        for byte in bytes {
                            array.push(Value::Number(byte as f64));
                        }

                        self.stack.push(Value::Array(array));
                    } else {
                        self.stack.push(Value::Null());
                    }
                }
            }
            12 => {
                // read file str
                let name;

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        name = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(platform) = &self.platform {
                    if let Some(bytes) = platform.read_file_bytes(name) {
                        let mut string = String::new();
                        for byte in bytes {
                            string.push(byte as char);
                        }

                        self.stack.push(Value::String(string));
                    } else {
                        self.stack.push(Value::Null());
                    }
                }
            }
            13 => {
                // write file bytes
                let name;
                let mut bytes: Vec<u8> = Vec::new();

                if let Some(value) = self.stack.pop() {
                    if let Value::Array(array) = value {
                        for value in array.iter() {
                            if let Value::Number(byte) = value {
                                bytes.push(*byte as u8);
                            } else {
                                self.error = "vmcall: expected non number in a byte array".into();
                            }
                        }
                    } else {
                        self.error = "vmcall: expected a byte array".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        name = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(platform) = &self.platform {
                    platform.write_file_bytes(name, bytes);
                }
            }
            14 => {
                // write file str
                let name;
                let mut bytes: Vec<u8> = Vec::new();

                if let Some(value) = self.stack.pop() {
                    if let Value::String(string) = value {
                        for ch in string.chars() {
                            bytes.push(ch as u8);
                        }
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
                        name = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if let Some(platform) = &self.platform {
                    platform.write_file_bytes(name, bytes);
                }
            }
            15 => {
                // ord
                let ch;

                if let Some(value) = self.stack.pop() {
                    if let Value::String(value) = value {
                        ch = value;
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                if ch.is_empty() {
                    self.stack.push(Value::Null());
                } else {
                    self.stack
                        .push(Value::Number(ch.chars().nth(0).unwrap() as i64 as f64));
                }
            }
            16 => {
                // chr
                let ch;

                if let Some(value) = self.stack.pop() {
                    if let Value::Number(value) = value {
                        ch = value as i64 as u8;
                    } else {
                        self.error = "vmcall: expected a number".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }

                self.stack
                    .push(Value::String(String::from_utf8_lossy(&[ch]).to_string()));
            }
            17 => {
                // stringify
                if let Some(value) = self.stack.pop() {
                    self.stack.push(Value::String(format!("{}", value)));
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }
            }
            18 => {
                // number
                if let Some(value) = self.stack.pop() {
                    if let Value::String(num_str) = value {
                        match num_str.parse::<f64>() {
                            Err(e) => {
                                self.stack.push(interop_err(Value::String(e.to_string())));
                            }
                            Ok(num) => {
                                self.stack.push(interop_ok(Value::Number(num)));
                            }
                        }
                    } else {
                        self.error = "vmcall: expected a string".into();
                        return;
                    }
                } else {
                    self.error = "vmcall: no value on stack".into();
                    return;
                }
            }
            _ => {
                if let Some(mut platform) = self.platform.take() {
                    let result = platform.as_mut().vmcall(self, index);
                    if !result {
                        self.error = format!("vmcall: invalid vmcall index {}", index);
                    }
                    self.platform = Some(platform);
                }
            }
        }
        */
    }
}
