use alloc::vec::Vec;

use crate::value::*;
use crate::vm::*;

impl VM {
    /// Check if a value is reachable
    pub(crate) fn gc_is_reachable(&mut self, ptr: *mut Object) -> bool {
        // Test if the value is in stack
        for value in self.stack.iter() {
            if let Value::Object(obj) = *value {
                if obj == ptr {
                    return true;
                }
            }
        }

        // Test if the value is in scopes
        for scope in self.scopes.iter() {
            for var in scope.vars.iter() {
                if let Value::Object(obj) = var.1 {
                    if obj == ptr {
                        return true;
                    }

                    unsafe {
                        if let Object::Array(array) = obj.read() {
                            for value in array.iter() {
                                if let Value::Object(obj) = value {
                                    if obj == ptr {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Unreachable
        return false;
    }

    /// Collects garbage
    pub fn gc(&mut self) {
        let mut ptrs = core::mem::take(&mut self.allocated_objs);
        let mut deleted_idxs: Vec<usize> = Vec::new();

        for (index, ptr) in ptrs.iter().enumerate() {
            if !self.gc_is_reachable(*ptr) {
                unsafe {
                    Object::free_and_drop(*ptr);
                }
                deleted_idxs.push(index);
            }
        }

        for index in deleted_idxs.iter().rev() {
            ptrs.remove(*index);
        }

        self.allocated_objs = ptrs;
    }
}
