//! Interop between ZenLang <-> Rust
//!
//! Used in zenlang::vm
use crate::value::*;
use alloc::collections::btree_map::BTreeMap;
use alloc::rc::*;
use alloc::string::*;
use core::cell::*;

/// Returns a result with an ok value
///
/// See ok() function in zenlang's stdlib
pub fn interop_ok(value: Value) -> Value {
    let mut result = BTreeMap::<String, Value>::new();
    result.insert("_ok".into(), value);
    result.insert("_err".into(), Value::Null());
    result.insert("_typename".into(), Value::String("Result".into()));
    let p = Rc::new(RefCell::new(Object::Dictionary(result)));
    return Value::Object(p);
}

/// Returns a result with an error value
///
/// See err() function in zenlang's stdlib
pub fn interop_err(value: Value) -> Value {
    let mut result = BTreeMap::<String, Value>::new();
    result.insert("_ok".into(), Value::Null());
    result.insert("_err".into(), value);
    result.insert("_typename".into(), Value::String("Result".into()));
    let p = Rc::new(RefCell::new(Object::Dictionary(result)));
    return Value::Object(p);
}

/// Get a value type string representation
pub fn get_type(value: &Value) -> String {
    return match &value {
        &Value::Number(_) => String::from("number"),
        &Value::String(_) => String::from("string"),
        &Value::Boolean(_) => String::from("boolean"),
        &Value::FunctionRef(_, _, _) => String::from("function"),
        &Value::Object(obj) => match &*obj.borrow() {
            Object::Array(_) => String::from("array"),
            Object::Dictionary(dict) => {
                if dict.contains_key("_typename") {
                    if let Value::String(str) = &dict["_typename"] {
                        return str.clone();
                    }
                }
                return "dict".to_string();
            }
        },
        &Value::Null() => String::from("null"),
    };
}
