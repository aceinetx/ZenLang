use crate::interop;
use crate::strong_u64::U64BitsControl;
use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_call(&mut self) {
        if let Some(value) = self.stack.pop() {
            if let Value::FunctionRef(addr, args_count, args_types) = value {
                if args_types.len() != args_count as usize && !args_types.is_empty() {
                    self.error = format!(
                        "call: invalid function reference: argument types length != argument count"
                    );
                }

                self.call_stack.push(self.pc);
                self.check_stack_overflow();
                self.pc = addr;
                self.pc.sub_low(1);
                self.add_scope();

                let this_name = &String::from("self");
                let scope = self.scopes.last_mut().unwrap();
                scope.create_if_doesnt_exist(this_name);
                *scope.get_mut(this_name).unwrap() = core::mem::take(&mut self.self_var);

                let start = self.bfas_stack_start.pop().unwrap();
                let end = self.bfas_stack_end.pop().unwrap();
                let diff = end - start;
                if diff != args_count as i64 {
                    self.error = format!(
                        "call: expected exactly {} arguments, but provided {} (trying to call a function at {}:{}) {:?}",
                        args_count,
                        diff,
                        self.pc.get_low(),
                        self.pc.get_high(),
                        self.stack
                    );
                    return;
                }

                if args_count > 0 && args_types.len() > 0 {
                    let from = start as usize;
                    let to = end as usize;
                    let slice = &self.stack[from..to];
                    for i in 0..slice.len() {
                        let value = &slice[i];
                        let expected = &args_types[i];
                        let actual = interop::get_type(value);

                        if actual != *expected && *expected != "" {
                            self.error = format!(
                                "call: expected {} as a {} argument but found {}",
                                expected, i, actual
                            );
                            return;
                        }
                    }
                }
            } else {
                self.error = "call: value on stack is not a function reference".into();
            }
        } else {
            self.error = "call: stack is empty".into();
        }
    }
}
