use crate::strong_u64::U64BitsControl;
use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_load_var(&mut self, name: &String) {
        if let Some(environ) = self.get_environ_by_id(*self.environs_stack.last().unwrap()) {
            if let Some(value) = environ.get(name) {
                self.stack.push(value.clone());
                return;
            }
        }

        if let Some(value) = self.global_scope.get(name) {
            self.stack.push(value.clone());
            return;
        }

        for module_i in 0..self.modules.len() {
            let module = &self.modules[module_i];
            for func in module.functions.iter() {
                if func.name.to_string() == name.to_string() {
                    let mut addr: u64 = 0;
                    addr.set_low(func.addr);
                    addr.set_high(module_i as u32);
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

            if let Some(environ) = self.get_environ_by_id_mut(*self.environs_stack.last().unwrap())
            {
                environ.create_if_doesnt_exist(name);
                if let Some(value) = environ.get_mut(name) {
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
