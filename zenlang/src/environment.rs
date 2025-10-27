//! Environment
use core::cell::RefCell;

use crate::value::Value;
use alloc::rc::Rc;
use alloc::string::*;
use alloc::vec::*;

/// Scope
///
/// Contains variables values and names
#[derive(Clone, Debug)]
pub struct Environment {
    pub(crate) vars: Vec<(String, Value)>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            vars: Vec::new(),
            parent: None,
        };
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

    /// Set a variable
    pub fn set(&mut self, name: &String, value: Value) {
        self.create_if_doesnt_exist(name);
        *self.get_mut(name).unwrap() = value;
    }

    /// Create a variable if doesn't exist
    pub fn create_if_doesnt_exist(&mut self, name: &String) {
        if self.get(name).is_none() {
            self.vars.push((name.to_string(), Value::Null()))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        return Environment::new();
    }
}
