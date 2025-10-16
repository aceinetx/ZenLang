use alloc::vec::Vec;

use crate::value::*;
use crate::vm::*;

impl VM {
    pub(crate) fn gc_is_reachable_array(&mut self, ptr: usize, array: &Object) -> bool {
        if let Object::Array(array) = array {
            for value in array.iter() {
                if let Value::Object(obj) = value {
                    if *obj == ptr {
                        return true;
                    }

                    if let Some(obj) = self.get_object(ptr) {
                        self.gc_is_reachable_array(ptr, obj);
                    }
                }
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

        // Test if the value is in scopes
        for scope in self.scopes.iter() {
            for var in scope.vars.iter() {
                if let Value::Object(obj) = var.1 {
                    if obj == ptr {
                        return true;
                    }

                    if let Some(obj) = self.get_object(ptr) {
                        self.gc_is_reachable_array(ptr, obj);
                    }
                }
            }
        }

        // Unreachable
        return false;
    }

    /// Collects garbage
    pub fn gc(&mut self) {
        let mut objs = core::mem::take(&mut self.objects);
        let mut deleted_idxs: Vec<usize> = Vec::new();

        for (index, _) in objs.iter().enumerate() {
            if !self.gc_is_reachable(index) {
                deleted_idxs.push(index);
            }
        }

        for index in deleted_idxs.iter().rev() {
            objs.remove(*index);
        }

        self.objects = objs;
    }

    pub fn free_all(&mut self) {
        self.objects.clear();
    }
}
