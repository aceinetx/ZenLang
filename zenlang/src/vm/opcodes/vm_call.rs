use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

impl VM {
    pub fn op_beginargs(&mut self) {
        self.args.push(Vec::new());
    }

    pub fn op_pusharg(&mut self) {
        match self.args.last_mut() {
            Some(args) => args,
            None => {
                self.error = "pusharg: beginargs wasn't called".into();
                return;
            }
        }
        .push(match self.stack.pop() {
            Some(value) => value,
            None => {
                self.error = "pusharg: no values on stack".into();
                return;
            }
        });
    }

    pub fn op_storearg(&mut self, name: &String) {
        let value = match match self.args.last_mut() {
            Some(args) => args,
            None => {
                self.error = "storearg: beginargs wasn't called".into();
                return;
            }
        }
        .pop()
        {
            Some(value) => value,
            None => {
                self.error = "storearg: no more arguments".into();
                return;
            }
        };

        if self.args.last().unwrap().is_empty() {
            self.args.pop();
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.create_if_doesnt_exist(&name);
        *scope.get_mut(&name).unwrap() = value;
    }

    pub fn op_call(&mut self) {
        if let Some(value) = self.stack.pop() {
            if let Value::FunctionRef(addr, args_count) = value {
                self.call_stack.push(self.pc);
                self.check_stack_overflow();
                self.pc = addr;
                self.pc.inst = self.pc.inst.wrapping_sub(1);
                self.add_scope();

                let this_name = &String::from("self");
                let scope = self.scopes.last_mut().unwrap();
                scope.create_if_doesnt_exist(this_name);
                *scope.get_mut(this_name).unwrap() = core::mem::take(&mut self.self_var);

                let diff = match self.args.last() {
                    Some(args) => args,
                    None => {
                        self.error = "call: beginargs wasn't called".into();
                        return;
                    }
                }
                .len();

                if diff != args_count {
                    self.error = format!(
                        "call: expected exactly {} arguments, but provided {} (trying to call a function at {})",
                        args_count, diff, self.pc,
                    );
                }
            } else {
                self.error = "call: value on stack is not a function reference".into();
            }
        } else {
            self.error = "call: stack is empty".into();
        }
    }
}
