use crate::ast::binop::*;
use crate::opcode::*;
use crate::strong_u64::U64BitsControl;
use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

impl VM {
    pub(crate) fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Call() => {
                if let Some(value) = self.stack.pop() {
                    if let Value::FunctionRef(addr, args_count) = value {
                        self.call_stack.push(self.pc);
                        self.check_stack_overflow();
                        self.pc = addr;
                        self.pc.sub_low(1);
                        self.add_scope();

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
                    } else {
                        self.error = "call: value on stack is not a function reference".into();
                    }
                } else {
                    self.error = "call: stack is empty".into();
                }
            }
            Opcode::Vmcall(index) => {
                self.vmcall(*index);
            }
            Opcode::Dynvmcall() => {
                let index;
                if let Some(value) = self.stack.pop() {
                    if let Value::Number(value) = value {
                        index = value as i64 as u8;
                    } else {
                        self.error = format!("dynvmcall failed: value on stack is not a number");
                        return;
                    }
                } else {
                    self.error = format!("dynvmcall failed: no more values on stack");
                    return;
                }
                self.vmcall(index);
            }
            Opcode::LoadConstant(value) => {
                let value = Value::Number(*value);
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::LoadNull() => {
                self.stack.push(Value::Null());
                self.check_stack_overflow();
            }
            Opcode::LoadBool(flag) => {
                self.stack.push(Value::Boolean(*flag));
                self.check_stack_overflow();
            }
            Opcode::LoadStr(value) => {
                let value = Value::String(value.to_string());
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::LoadVar(name) => {
                // do something with the clone here
                if let Some(scope) = self.scopes.last() {
                    if let Some(value) = scope.get(name) {
                        self.stack.push(value.clone());
                        self.check_stack_overflow();
                        return;
                    }
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
            Opcode::StoreVar(name) => {
                // do something with the clone here
                if let Some(store_value) = self.stack.pop() {
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
            Opcode::PushRet() => {
                // do smth with the clone
                self.stack.push(self.ret.clone());
            }
            Opcode::Cafse(items) => {
                let mut vec = Vec::<Value>::new();
                for _ in 0..*items {
                    if let Some(stack_value) = self.stack.pop() {
                        vec.insert(0, stack_value);
                    } else {
                        self.error = format!("cafse failed: no more values on stack");
                        return;
                    }
                }
                let obj = Object::Array(vec);
                let v = Value::Object(self.add_object(obj));
                self.stack.push(v);
            }
            Opcode::Iafs() => {
                let array;
                let index;
                if let Some(value) = self.stack.pop() {
                    index = value;
                } else {
                    self.error = format!("iafs failed: no more values on stack for index");
                    return;
                }
                if let Some(value) = self.stack.pop() {
                    array = value;
                } else {
                    self.error = format!("iafs failed: no more values on stack for array");
                    return;
                }

                match array {
                    Value::Object(obj) => match self.get_object(obj) {
                        Some(Object::Array(array)) => {
                            if let Value::Number(index) = index {
                                let usize_index = index as usize;
                                if usize_index >= array.len() {
                                    self.stack.push(Value::Null());
                                    return;
                                }

                                self.stack.push(array[usize_index].clone());
                                return;
                            }
                        }
                        Some(Object::Dictionary(dict)) => {
                            if let Value::String(index) = index {
                                for element in dict.iter() {
                                    if element.0 == index {
                                        self.stack.push(element.1.clone());
                                        return;
                                    }
                                }
                                self.stack.push(Value::Null());
                                return;
                            }
                        }
                        _ => {
                            self.error =
                                format!("iafs failed: invalid reference: referencing 0x{:?}", obj);
                        }
                    },
                    Value::String(string) => {
                        if let Value::Number(index) = index {
                            if let Some(ch) = string.chars().nth(index as usize) {
                                self.stack.push(Value::String(String::from(ch)));
                            } else {
                                self.stack.push(Value::Null());
                            }
                            return;
                        }
                    }
                    _ => {
                        self.error = format!(
                            "iafs failed: invalid operand types: {:?} {:?}",
                            array, index
                        );
                    }
                }
            }
            Opcode::Cdfse(names) => {
                let mut items = Vec::<(String, Value)>::new();
                for i in 0..names.len() {
                    if let Some(stack_value) = self.stack.pop() {
                        // todo: do smth w ts clone
                        items.insert(0, (names[names.len() - i - 1].clone(), stack_value));
                    } else {
                        self.error = format!("cdfse failed: no more values on stack");
                        return;
                    }
                }

                let obj = Object::Dictionary(items);
                let v = Value::Object(self.add_object(obj));
                self.stack.push(v);
            }
            Opcode::Aiafs() => {
                let set_value: Value;
                let index: Value;
                let mut object: Value;

                if let Some(value) = self.stack.pop() {
                    index = value;
                } else {
                    self.error = format!("aiafs failed: no more values on stack for index");
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    set_value = value;
                } else {
                    self.error = format!("aiafs failed: no more values on stack for set value");
                    return;
                }

                if let Some(value) = self.stack.pop() {
                    object = value;
                } else {
                    self.error = format!("aiafs failed: no more values on stack for object");
                    return;
                }

                self.aiafs(&mut object, set_value, index);
            }
            Opcode::BeginFnArgs() => {
                self.bfas_stack_start.push(self.stack.len() as i64);
            }
            Opcode::EndFnArgs() => {
                self.bfas_stack_end.push(self.stack.len() as i64);
            }
            Opcode::Pop() => {
                if self.stack.is_empty() {
                    self.error = format!("pop failed: no value in stack");
                } else {
                    self.stack.pop();
                }
            }
            Opcode::BranchTrue(addr) => {
                if let Some(value) = self.stack.pop() {
                    if let Value::Boolean(flag) = value {
                        if flag {
                            self.pc.set_low(*addr - 1);
                        }
                        return;
                    }
                    if let Value::Number(num) = value {
                        if num != 0.0 {
                            self.pc.set_low(*addr - 1);
                        }
                        return;
                    }

                    self.error = "bst failed: value on stack is not of an acceptable type".into();
                } else {
                    self.error = "bst failed: no value on stack".into();
                }
            }
            Opcode::BranchNonNull(addr) => {
                if let Some(value) = self.stack.pop() {
                    if let Value::Null() = value {
                        return;
                    }
                    self.pc.set_low(*addr);
                    self.pc.sub_low(1);
                } else {
                    self.error = "bsnn failed: no value on stack".into();
                }
            }
            Opcode::Branch(addr) => {
                self.pc.set_low(*addr);
                self.pc.sub_low(1);
            }
            Opcode::Add() => {
                let value = self.compute_stack_values(AstBinopOp::PLUS);
                self.stack.push(value);
            }
            Opcode::Sub() => {
                let value = self.compute_stack_values(AstBinopOp::MINUS);
                self.stack.push(value);
            }
            Opcode::Mul() => {
                let value = self.compute_stack_values(AstBinopOp::MUL);
                self.stack.push(value);
            }
            Opcode::Div() => {
                let value = self.compute_stack_values(AstBinopOp::DIV);
                self.stack.push(value);
            }
            Opcode::Eq() => {
                let value = self.compute_stack_values(AstBinopOp::EQ);
                self.stack.push(value);
            }
            Opcode::Neq() => {
                let value = self.compute_stack_values(AstBinopOp::NEQ);
                self.stack.push(value);
            }
            Opcode::Lt() => {
                let value = self.compute_stack_values(AstBinopOp::LT);
                self.stack.push(value);
            }
            Opcode::Gt() => {
                let value = self.compute_stack_values(AstBinopOp::GT);
                self.stack.push(value);
            }
            Opcode::Le() => {
                let value = self.compute_stack_values(AstBinopOp::LE);
                self.stack.push(value);
            }
            Opcode::Ge() => {
                let value = self.compute_stack_values(AstBinopOp::GE);
                self.stack.push(value);
            }
            Opcode::Bshr() => {
                let value = self.compute_stack_values(AstBinopOp::BITSHR);
                self.stack.push(value);
            }
            Opcode::Bshl() => {
                let value = self.compute_stack_values(AstBinopOp::BITSHL);
                self.stack.push(value);
            }
            Opcode::Band() => {
                let value = self.compute_stack_values(AstBinopOp::BITAND);
                self.stack.push(value);
            }
            Opcode::Bor() => {
                let value = self.compute_stack_values(AstBinopOp::BITOR);
                self.stack.push(value);
            }
            Opcode::Ret() => {
                if !self.stack.is_empty() {
                    self.ret = self.stack.pop().unwrap();
                }

                self.remove_scope();

                if !self.call_stack.is_empty() {
                    self.pc = self.call_stack.pop().unwrap();
                } else {
                    self.pc.set_high(u32::MAX);

                    if let Value::Object(ret) = self.ret {
                        if self.get_object(ret).is_some() {
                            self.remove_object(ret);
                        }
                    }
                }
            }
        }
    }
}
