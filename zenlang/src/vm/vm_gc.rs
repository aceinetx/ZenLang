use alloc::vec::Vec;

use crate::value::*;
use crate::vm::*;

impl VM {
    pub(crate) fn gc_is_reachable_obj(&mut self, ptr: usize, array_ptr: usize) -> bool {
        let mut test_reachable: Vec<usize> = Vec::new();
        if let Some(array) = self.get_object(array_ptr) {
            if let Object::Array(array) = array {
                for value in array.iter() {
                    if let Value::Object(obj) = value {
                        if *obj == ptr {
                            return true;
                        }

                        test_reachable.push(*obj);
                    }
                }
            } else if let Object::Dictionary(dict) = array {
                for pair in dict.iter() {
                    if let Value::Object(obj) = pair.1 {
                        if obj == ptr {
                            return true;
                        }

                        test_reachable.push(obj);
                    }
                }
            }
        }

        for p in test_reachable.iter() {
            if self.gc_is_reachable_obj(ptr, *p) {
                return true;
            }
        }
        return false;
    }

    /// Check if a value is reachable
    pub(crate) fn gc_is_reachable(&mut self, ptr: usize) -> bool {
        // Test if the value is in stack
        for value in self.stack.iter() {
            if let Value::Object(obj) = *value {
                if obj == ptr {
                    return true;
                }
            }
        }

        // In ret?
        if let Value::Object(obj) = self.ret {
            if obj == ptr {
                return true;
            }
        }

        // Test if the value is in scopes
        let scopes = core::mem::take(&mut self.scopes);
        for scope in scopes.iter() {
            for var in scope.vars.iter() {
                if let Value::Object(obj) = var.1 {
                    if obj == ptr {
                        self.scopes = scopes;
                        return true;
                    }

                    if self.gc_is_reachable_obj(ptr, obj) {
                        self.scopes = scopes;
                        return true;
                    }
                }
            }
        }
        self.scopes = scopes;

        // Test if the value is in global scope
        if !self.halted {
            let global_scope = core::mem::take(&mut self.global_scope);
            for var in global_scope.vars.iter() {
                if let Value::Object(obj) = var.1 {
                    if obj == ptr {
                        self.global_scope = global_scope;
                        return true;
                    }

                    if self.gc_is_reachable_obj(ptr, obj) {
                        self.global_scope = global_scope;
                        return true;
                    }
                }
            }
            self.global_scope = global_scope;
        }

        // Unreachable
        return false;
    }

    /// Collects garbage
    pub fn gc(&mut self) {
        let mut deleted_ptrs: Vec<usize> = Vec::new();
        let mut ptrs: Vec<usize> = Vec::new();

        for (ptr, _) in self.objects.iter() {
            ptrs.push(*ptr);
        }

        for ptr in ptrs.iter() {
            if !self.gc_is_reachable(*ptr) {
                deleted_ptrs.push(*ptr);
            }
        }

        for ptr in deleted_ptrs.iter() {
            self.remove_object(*ptr);
        }
    }
}
