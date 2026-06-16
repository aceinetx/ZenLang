use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

impl VM {
    pub fn op_beginargs(&mut self) -> Result<(), VMError> {
        self.args.push(Vec::new());
        Ok(())
    }

    pub fn op_pusharg(&mut self) -> Result<(), VMError> {
        match self.args.last_mut() {
            Some(args) => args,
            None => {
                return Err("pusharg: beginargs wasn't called".into());
            }
        }
        .push(match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("pusharg: no values on stack".into());
            }
        });
        return Ok(());
    }

    pub fn op_storearg(&mut self, name: &String) -> Result<(), VMError> {
        let value = match match self.args.last_mut() {
            Some(args) => args,
            None => {
                return Err("storearg: beginargs wasn't called".into());
            }
        }
        .pop()
        {
            Some(value) => value,
            None => {
                return Err("storearg: no more arguments".into());
            }
        };

        if self.args.last().unwrap().is_empty() {
            self.args.pop();
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.create_if_doesnt_exist(&name);
        *scope.get_mut(&name).unwrap() = value;

        return Ok(());
    }

    pub fn op_call(&mut self) -> Result<(), VMError> {
        let value = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("call: stack is empty".into());
            }
        };

        match value {
            Value::FunctionRef(addr, args_count) => {
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
                        return Err("call: beginargs wasn't called".into());
                    }
                }
                .len();

                if diff != args_count {
                    return Err(format!(
                        "call: expected exactly {} arguments, but provided {} (trying to call a function at {})",
                        args_count, diff, self.pc,
                    ));
                }
            }
            Value::Lambda(pc, scope, args_count) => {
                self.call_stack.push(self.pc);
                self.check_stack_overflow();
                self.pc = pc;
                self.pc.inst = self.pc.inst.wrapping_sub(1);

                self.scopes.push((&*scope.borrow()).clone());

                let diff = match self.args.last() {
                    Some(args) => args,
                    None => {
                        return Err("call: beginargs wasn't called".into());
                    }
                }
                .len();

                if diff != args_count {
                    return Err(format!(
                        "call: expected exactly {} arguments, but provided {} (trying to call a lambda at {})",
                        args_count, diff, self.pc,
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "call: value on stack is not a function reference or a lambda ({})",
                    value.get_type()
                ));
            }
        }
        return Ok(());
    }
}
