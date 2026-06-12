use crate::value::*;
use crate::vm::ProgramCounter;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_load_var(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last() {
            if let Some(value) = scope.get(name) {
                self.stack.push(value.clone());
                self.check_stack_overflow();
                return;
            }
        }

        if let Some(value) = self.global_scope.get(name) {
            self.stack.push(value.clone());
            self.check_stack_overflow();
            return;
        }

        for module_i in 0..self.modules.len() {
            let module = &self.modules[module_i];
            for func in module.functions.iter() {
                if func.name == *name {
                    let mut addr = ProgramCounter::new();
                    addr.inst = func.addr;
                    addr.module = module_i;
                    self.stack.push(Value::FunctionRef(addr, func.args_count));
                    self.check_stack_overflow();
                    return;
                }
            }
        }
        self.stack.push(Value::Null());
    }

    pub fn op_store_var(&mut self, name: &String) {
        // do something with the clone here
        if let Some(store_value) = self.stack.pop() {
            if let Some(value) = self.global_scope.get_mut(name) {
                *value = store_value;
                return;
            }

            if let Some(scope) = self.scopes.last_mut() {
                scope.create_if_doesnt_exist(name);
                if let Some(value) = scope.get_mut(name) {
                    *value = store_value;
                    return;
                }
            } else {
                self.error = format!("storev failed: scopes is empty");
                return;
            }
        } else {
            self.error = format!("storev failed: no value in stack");
            return;
        }
    }
}
