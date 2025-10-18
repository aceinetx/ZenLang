//! Interop between ZenLang <-> Rust
//!
//! Used in zenlang::vm
use crate::value::*;
use alloc::rc::*;
use alloc::string::*;
use alloc::vec::*;
use core::cell::*;

/// Returns a result with an ok value
///
/// See ok() function in zenlang's stdlib
pub fn interop_ok(value: Value) -> Value {
    let mut result: Vec<(String, Value)> = Vec::new();
    result.push(("_ok".into(), value));
    result.push(("_err".into(), Value::Null()));
    let p = Rc::new(RefCell::new(Object::Dictionary(result)));
    return Value::Object(p);
}

/// Returns a result with an error value
///
/// See err() function in zenlang's stdlib
pub fn interop_err(value: Value) -> Value {
    let mut result: Vec<(String, Value)> = Vec::new();
    result.push(("_ok".into(), Value::Null()));
    result.push(("_err".into(), value));
    let p = Rc::new(RefCell::new(Object::Dictionary(result)));
    return Value::Object(p);
}
