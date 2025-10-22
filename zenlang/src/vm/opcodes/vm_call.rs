use crate::strong_u64::U64BitsControl;
use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::string::*;

impl VM {
    pub fn op_call(&mut self) {
        if let Some(value) = self.stack.pop() {
            match value {
                Value::FunctionRef(addr, args_count) => {
                    self.call_stack.push(self.pc);
                    self.check_stack_overflow();
                    self.pc = addr;
                    self.pc.sub_low(1);

                    self.push_environment();

                    let start = self.bfas_stack_start.pop().unwrap();
                    let end = self.bfas_stack_end.pop().unwrap();
                    let diff = end - start;
                    if diff != args_count as i64 {
                        self.error = format!(
                            "call: expected exactly {} arguments, but provided {} (trying to call a function at {}:{})",
                            args_count,
                            diff,
                            self.pc.get_low(),
                            self.pc.get_high(),
                        );
                    }
                }
                Value::FunctionRefEnv(addr, args_count, env) => {
                    self.call_stack.push(self.pc);
                    self.check_stack_overflow();
                    self.pc = addr;

                    self.environs.push(env);

                    let start = self.bfas_stack_start.pop().unwrap();
                    let end = self.bfas_stack_end.pop().unwrap();
                    let diff = end - start;
                    if diff != args_count as i64 {
                        self.error = format!(
                            "call: expected exactly {} arguments, but provided {} (trying to call an env function at {}:{})",
                            args_count,
                            diff,
                            self.pc.get_low(),
                            self.pc.get_high(),
                        );
                    }
                }
                _ => {
                    self.error = format!(
                        "call: value on stack is not a function reference ({})",
                        value
                    );
                }
            }
        } else {
            self.error = "call: stack is empty".into();
        }
    }
}
