use crate::value::*;
use crate::vm::ProgramCounter;
use crate::vm::*;
use alloc::string::*;

impl VM {
    pub fn op_load_var(&mut self, name: &String) -> Result<(), VMError> {
        if let Some(scope) = self.scopes.last() {
            if let Some(value) = scope.get(name) {
                self.stack.push(value.clone());
                return self.check_stack_overflow();
            }
        }

        if let Some(value) = self.global_scope.get(name) {
            self.stack.push(value.clone());
            return self.check_stack_overflow();
        }

        for module_i in 0..self.modules.len() {
            let module = &self.modules[module_i];
            for func in module.functions.iter() {
                if func.name == *name {
                    let mut addr = ProgramCounter::new();
                    addr.inst = func.addr;
                    addr.module = module_i;
                    self.stack.push(Value::FunctionRef(addr, func.args_count));
                    return self.check_stack_overflow();
                }
            }
        }
        self.stack.push(Value::Null());
        return Ok(());
    }

    pub fn op_store_var(&mut self, name: &String) -> Result<(), VMError> {
        // do something with the clone here
        let store_value = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("storev failed: no value in stack".into());
            }
        };

        if let Some(value) = self.global_scope.get_mut(name) {
            *value = store_value;
            return Ok(());
        }

        let scope = match self.scopes.last_mut() {
            Some(scope) => scope,
            None => {
                return Err("storev failed: scopes is empty".into());
            }
        };

        scope.create_if_doesnt_exist(name);
        *scope.get_mut(name).unwrap() = store_value;

        return Ok(());
    }
}
