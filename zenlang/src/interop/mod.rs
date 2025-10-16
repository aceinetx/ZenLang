//! Interop between ZenLang <-> Rust
//!
//! Used in zenlang::vm
use crate::value::*;
use alloc::string::*;
use alloc::vec::*;

/// Returns a result with an ok value
///
/// See ok() function in zenlang's stdlib
pub fn interop_ok(value: Value) -> Value {
    let mut result: Vec<(String, Value)> = Vec::new();
    result.push(("_ok".into(), value));
    result.push(("_err".into(), Value::Null()));
    //let p = Object::Dictionary(&result);
    return Value::Object(1);
}

/// Returns a result with an error value
///
/// See err() function in zenlang's stdlib
pub fn interop_err(value: Value) -> Value {
    let mut result: Vec<(String, Value)> = Vec::new();
    result.push(("_ok".into(), Value::Null()));
    result.push(("_err".into(), value));
    return Value::Object(1);
}
