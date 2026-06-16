use core::cell::RefCell;

use crate::interop::*;
use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::*;
use alloc::vec::*;

macro_rules! pop_stack {
    ($stack: expr) => {
        match $stack.pop() {
            Some(value) => value,
            None => {
                return Err("vmcall: no value on stack".into());
            }
        }
    };
}

macro_rules! pop_value {
    ($stack: expr, $pat: path) => {
        match pop_stack!($stack) {
            $pat(v) => v,
            _ => {
                return Err(format!("vmcall: expected {}", stringify!($pat)));
            }
        }
    };
}

macro_rules! pop_object_pair {
    ($stack: expr, $pat: path) => {{
        let obj = pop_value!($stack, Value::Object);
        match &*obj.borrow_mut() {
            $pat(v) => (obj, v),
            _ => {
                return Err(format!("vmcall: expected {}", stringify!($pat)));
            }
        }
    }};
}

macro_rules! pop_object {
    ($stack: expr, $pat: path) => {
        pop_object_pair!($stack, $pat).1
    };
}

impl VM {
    /// Performs a vmcall
    ///
    /// ### VMCall indexes
    /// - 1: print
    /// - 2: println
    /// - 3: get_string
    pub fn vmcall(&mut self, index: u8) -> Result<(), VMError> {
        match index {
            1 => {
                // print
                if let Some(platform) = &self.platform {
                    platform.print(format!("{}", pop_stack!(self.stack)));
                }
            }
            2 => {
                // println
                if let Some(platform) = &self.platform {
                    platform.println(format!("{}", pop_stack!(self.stack)));
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
                    let name = pop_value!(self.stack, Value::String);

                    if let Some(module) = platform.get_module(name) {
                        let _ = self.load_module(&module);
                    } else {
                        return Err(format!("module not found: {}", name));
                    }
                    return Ok(());
                }
            }
            5 => {
                // array size
                let array = pop_object!(self.stack, Object::Array);

                self.stack.push(Value::Number(array.len() as f64));
            }
            6 => {
                // array push
                let element;

                let value = pop_stack!(self.stack);
                let (obj, array) = pop_object_pair!(self.stack, Object::Array);

                array.push(element);
                self.stack.push(Value::Object(obj));
            }
            7 => {
                // array pop
                let (obj, array) = pop_object_pair!(self.stack, Object::Array);

                array.pop();
                self.stack.push(Value::Object(obj));
            }
            8 => {
                // array remove
                let value = pop_stack!(self.stack);
                let at = pop_value!(self.stack, Value::Number) as usize;
                let (obj, array) = pop_object_pair!(self.stack, Object::Array);

                if at >= array.len() {
                    self.stack.push(Value::Null());
                    return Ok(());
                }

                array.remove(at);
                self.stack.push(Value::Object(obj));
            }
            9 => {
                // array insert
                let element = pop_stack!(self.stack);
                let at = pop_value!(self.stack, Value::Number) as usize;
                let (obj, array) = pop_object_pair!(self.stack, Object::Array);

                if at >= array.len() {
                    self.stack.push(Value::Null());
                    return Ok(());
                }

                array.insert(at, element);
                self.stack.push(Value::Object(obj));
            }
            10 => {
                // string split
                let delimiter = pop_value!(self.stack, Value::String);
                let string = pop_value!(self.stack, Value::String);

                let mut array: Vec<Value> = Vec::new();
                for part in string.split(&delimiter) {
                    array.push(Value::String(String::from(part)))
                }

                let ptr = Rc::new(RefCell::new(Object::Array(array)));
                self.stack.push(Value::Object(ptr));
            }
            11 => {
                // read file bytes
                let name = pop_value!(self.stack, Value::String);

                if let Some(platform) = &self.platform {
                    if let Some(bytes) = platform.read_file_bytes(name) {
                        let mut array: Vec<Value> = Vec::new();
                        for byte in bytes {
                            array.push(Value::Number(byte as f64));
                        }

                        let ptr = Rc::new(RefCell::new(Object::Array(array)));
                        self.stack.push(Value::Object(ptr));
                    } else {
                        self.stack.push(Value::Null());
                    }
                }
            }
            12 => {
                // read file str
                let name = pop_value!(self.stack, Value::String);

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
                let mut bytes: Vec<u8> = Vec::new();

                let array = pop_object!(self.stack, Object::Array);

                for value in array.iter() {
                    if let Value::Number(byte) = value {
                        bytes.push(*byte as u8);
                    } else {
                        return Err("vmcall: found non number in a byte array".into());
                    }
                }

                let name = pop_value!(self.stack, Value::String);

                if let Some(platform) = &self.platform {
                    platform.write_file_bytes(name, bytes);
                }
            }
            14 => {
                // write file str
                let mut bytes: Vec<u8> = Vec::new();

                let string = pop_value!(self.stack, Value::String);
                for ch in string.chars() {
                    bytes.push(ch as u8);
                }
                let name = pop_value!(self.stack, Value::String);

                if let Some(platform) = &self.platform {
                    platform.write_file_bytes(name, bytes);
                }
            }
            15 => {
                // ord
                let ch = pop_value!(self.stack, Value::String);

                if ch.is_empty() {
                    self.stack.push(Value::Null());
                } else {
                    self.stack
                        .push(Value::Number(ch.chars().nth(0).unwrap() as i64 as f64));
                }
            }
            16 => {
                // chr
                let ch = pop_value!(self.stack, Value::Number) as i64 as u8;

                self.stack
                    .push(Value::String(String::from_utf8_lossy(&[ch]).to_string()));
            }
            17 => {
                // stringify
                self.stack
                    .push(Value::String(format!("{}", pop_stack!(self.stack))));
            }
            18 => {
                // number
                let num_str = pop_value!(self.stack, Value::String);
                match num_str.parse::<f64>() {
                    Err(e) => {
                        self.stack.push(interop_err(Value::String(e.to_string())));
                    }
                    Ok(num) => {
                        self.stack.push(interop_ok(Value::Number(num)));
                    }
                }
            }
            19 => {
                // clone
                let obj = &*pop_value!(self.stack, Value::Object).borrow();

                let new = Rc::new(RefCell::new(obj.clone()));
                self.stack.push(Value::Object(new));
            }
            _ => {
                if let Some(mut platform) = self.platform.take() {
                    let result = platform.as_mut().vmcall(self, index);
                    if result.is_none() {
                        return Err(format!("vmcall: invalid vmcall index {}", index));
                    }
                    self.platform = Some(platform);
                }
                return Ok(());
            }
        }
        return Ok(());
    }
}
