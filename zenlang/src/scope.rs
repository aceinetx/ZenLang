use crate::value::Value;
use alloc::string::*;
use alloc::vec::*;

#[derive(Debug)]
pub struct Scope {
    vars: Vec<(String, Value)>,
}

impl Scope {
    pub fn new() -> Scope {
        return Scope { vars: Vec::new() };
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        for var in self.vars.iter() {
            if var.0 == name.to_string() {
                return Some(&var.1);
            }
        }
        return None;
    }

    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        for var in self.vars.iter_mut() {
            if var.0 == name.to_string() {
                return Some(&mut var.1);
            }
        }
        return None;
    }

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
