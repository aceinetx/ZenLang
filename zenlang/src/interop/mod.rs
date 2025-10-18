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
