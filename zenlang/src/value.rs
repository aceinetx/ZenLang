use crate::strong_u64::U64BitsControl;
use alloc::string::*;
use alloc::vec::*;
use core::fmt::Display;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    FunctionRef(u64, u64),
    Null(),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for i in 0..a.len() {
                    if a[i] != b[i] {
                        return false;
                    }
                }
                return true;
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
            Value::Array(array) => {
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
