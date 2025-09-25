//! Scope
//!
//! Contains: Scope

use crate::value::Value;
use alloc::string::*;
use alloc::vec::*;

/// Scope
///
/// Contains variables values and names
#[derive(Debug)]
pub struct Scope {
    vars: Vec<(String, Value)>,
}

impl Scope {
    pub fn new() -> Scope {
        return Scope { vars: Vec::new() };
    }

    /// Get a variable reference by a name
    pub fn get(&self, name: &String) -> Option<&Value> {
        for var in self.vars.iter() {
            if var.0 == name.to_string() {
                return Some(&var.1);
            }
        }
        return None;
    }

    /// Get a variable mutable reference by a name
    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        for var in self.vars.iter_mut() {
            if var.0 == name.to_string() {
                return Some(&mut var.1);
            }
        }
        return None;
    }

    /// Create a variable if doesn't exist
    pub fn create_if_doesnt_exist(&mut self, name: &String) {
        if self.get(name).is_none() {
            self.vars.push((name.to_string(), Value::Null()))
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        return Scope::new();
    }
}
