//! Value
//!
//! ZenLang variable value
use crate::strong_u64::U64BitsControl;
use crate::vm::VM;
use alloc::string::*;
use alloc::vec::*;
use core::fmt::Display;

/// Object
#[derive(Clone, Debug)]
pub enum Object {
    Array(Vec<Value>),
    Dictionary(Vec<(String, Value)>),
}

/// Value
#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    FunctionRef(u64, u64),
    Object(usize),
    Null(),
}

impl Value {
    /// Perform a less than (<) operation. False if unsupported operands
    pub fn lt(&self, other: &Value) -> bool {
        if let Value::Number(a) = self {
            if let Value::Number(b) = other {
                return a < b;
            }
        }
        return false;
    }

    /// Perform a greater than (>) operation. False if unsupported operands
    pub fn gt(&self, other: &Value) -> bool {
        if let Value::Number(a) = self {
            if let Value::Number(b) = other {
                return a > b;
            }
        }
        return false;
    }

    /// Perform a less than or equal (<=) operation. False if unsupported operands
    pub fn le(&self, other: &Value) -> bool {
        if let Value::Number(a) = self {
            if let Value::Number(b) = other {
                return a <= b;
            }
        }
        return false;
    }

    /// Perform a greater than or equal (>=) operation. False if unsupported operands
    pub fn ge(&self, other: &Value) -> bool {
        if let Value::Number(a) = self {
            if let Value::Number(b) = other {
                return a >= b;
            }
        }
        return false;
    }

    /// Perform an equal (==) operation. All operands are valid
    pub fn equal(&self, other: &Value, vm: &VM) -> bool {
        match (self, other) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Object(obja), Value::Object(objb)) => {
                match (vm.get_object(*obja), vm.get_object(*objb)) {
                    (Some(Object::Array(a)), Some(Object::Array(b))) => {
                        if a.len() != b.len() {
                            return false;
                        }
                        for i in 0..a.len() {
                            if !a[i].equal(&b[i], vm) {
                                return false;
                            }
                        }
                        return true;
                    }
                    (Some(Object::Dictionary(a)), Some(Object::Dictionary(b))) => {
                        if a.len() != b.len() {
                            return false;
                        }

                        for i in 0..a.len() {
                            if a[i].0 != b[i].0 {
                                return false;
                            }
                            if !a[i].1.equal(&b[i].1, vm) {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => return false,
                }
            }
            (Value::FunctionRef(a, b), Value::FunctionRef(c, d)) => {
                return a == c && b == d;
            }
            (Value::Null(), Value::Null()) => {
                return true;
            }
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Number(num) => {
                return write!(f, "{}", num);
            }
            Value::String(string) => {
                return write!(f, "{}", string);
            }
            Value::Boolean(boolean) => {
                return write!(f, "{}", boolean);
            }
            Value::Object(obj) => {
                return write!(f, "[object at 0x{:x}]", obj);
                /*                match obj.read() {
                    Object::Array(array) => {
                        let _ = write!(f, "[");

                        let len = array.len();
                        for i in 0..len {
                            if let Value::String(_) = array[i] {
                                let _ = write!(f, "\"{}\"", array[i]);
                            } else {
                                let _ = write!(f, "{}", array[i]);
                            }

                            if i != len - 1 {
                                let _ = write!(f, ", ");
                            }
                        }

                        let _ = write!(f, "]");
                        Ok(())
                    }
                    Object::Dictionary(dict) => {
                        let _ = write!(f, "{{");

                        let len = dict.len();
                        for i in 0..len {
                            let entry = &dict[i];
                            let _ = write!(f, "{} = ", entry.0);

                            if let Value::String(_) = entry.1 {
                                let _ = write!(f, "\"{}\"", entry.1);
                            } else {
                                let _ = write!(f, "{}", entry.1);
                            }

                            if i != len - 1 {
                                let _ = write!(f, ", ");
                            }
                        }

                        let _ = write!(f, "}}");
                        Ok(())
                    }
                }
                */
            }
            Value::FunctionRef(addr, args_count) => {
                return write!(
                    f,
                    "[function at 0x{:?} in module {} with {} arguments]",
                    addr.get_low(),
                    addr.get_high(),
                    args_count
                );
            }
            Value::Null() => {
                return write!(f, "null");
            }
        }
    }
}
