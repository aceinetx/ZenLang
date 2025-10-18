//! Value
//!
//! ZenLang variable value
use crate::strong_u64::U64BitsControl;
use crate::vm::VM;
use alloc::collections::btree_map::BTreeMap;
use alloc::rc::*;
use alloc::string::*;
use alloc::vec::*;
use core::cell::RefCell;
use core::fmt::Display;

/// Object
#[derive(Clone, Debug)]
pub enum Object {
    Array(Vec<Value>),
    Dictionary(BTreeMap<String, Value>),
}

/// Value
#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    FunctionRef(u64, u64),
    Object(Rc<RefCell<Object>>),
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
                match (&*obja.borrow(), &*objb.borrow()) {
                    (Object::Array(a), Object::Array(b)) => {
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
                    (Object::Dictionary(a), Object::Dictionary(b)) => {
                        if a.len() != b.len() {
                            return false;
                        }

                        for pair in a.iter() {
                            if !b.contains_key(pair.0) {
                                return false;
                            }
                            if !pair.1.equal(&b[pair.0], vm) {
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
                //return write!(f, "[object at 0x{:x}]", obj.as_ptr() as u64);
                match &*obj.borrow() {
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
                        if dict.contains_key("_typename".into()) {
                            let _ = write!(f, "{} ", dict.get("_typename").unwrap());
                        }

                        let _ = write!(f, "{{");

                        if dict.len() > 0 {
                            let first = dict.iter().nth(0).unwrap();

                            for pair in dict.iter() {
                                if pair.0 == "_typename" {
                                    continue;
                                }

                                if pair.0 != first.0 {
                                    let _ = write!(f, ", ");
                                }

                                let _ = write!(f, "{} = ", pair.0);

                                if let Value::String(_) = pair.1 {
                                    let _ = write!(f, "\"{}\"", pair.1);
                                } else {
                                    let _ = write!(f, "{}", pair.1);
                                }
                            }
                        }

                        let _ = write!(f, "}}");
                        Ok(())
                    }
                }
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
