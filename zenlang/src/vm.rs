use crate::ast::binop::AstBinopOp;
use crate::module::Module;
use crate::opcode::Opcode;
use crate::platform::Platform;
use crate::scope::Scope;
use crate::strong_u64::*;
use crate::value::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

static MAX_STACK_SIZE: usize = 1000;

pub struct VM<'a> {
    pub modules: Vec<&'a mut Module>,
    pub pc: u64,
    pub stack: Vec<Value>,
    pub call_stack: Vec<u64>,
    pub scopes: Vec<Scope>,
    pub error: String,
    pub ret: Value,
    pub platform: Option<Box<dyn Platform>>,
    bfas_stack_start: i64,
    bfas_stack_end: i64,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        return VM {
            modules: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            scopes: Vec::new(),
            error: String::new(),
            ret: Value::Null(),
            platform: None,
            bfas_stack_start: 0,
            bfas_stack_end: 0,
        };
    }

    pub fn load_module(&mut self, module: &'a mut Module) {
        self.modules.push(module);
    }

    pub fn set_entry_function(&mut self, entry_fn_name: &str) -> Result<(), &'static str> {
        for i in 0..self.modules.len() {
            let module = &self.modules[i];
            for function in module.functions.iter() {
                if function.name == entry_fn_name {
                    self.pc.set_low(function.addr as u32);
                    self.pc.set_high(i as u32);
                    self.add_scope();
                    return Ok(());
                }
            }
        }
        return Err("cannot find entry function");
    }

    fn compute_values(&mut self, left: Value, right: Value, op: AstBinopOp) -> Value {
        match op {
            AstBinopOp::PLUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num + right_num);
                    }
                }
            }
            AstBinopOp::MINUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num - right_num);
                    }
                }
            }
            AstBinopOp::MUL => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num * right_num);
                    }
                }
            }
            AstBinopOp::DIV => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        if right_num == 0.0 {
                            self.error = "division by 0".into();
                            return Value::Null();
                        }
                        return Value::Number(left_num / right_num);
                    }
                }
            }
            AstBinopOp::BITSHR => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) >> (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::BITSHL => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) << (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::BITAND => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) & (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::BITOR => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) | (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::EQ => {
                return Value::Boolean(left.equal(&right));
            }
            AstBinopOp::NEQ => {
                return Value::Boolean(!left.equal(&right));
            }
            AstBinopOp::LT => {
                return Value::Boolean(left.lt(&right));
            }
            AstBinopOp::GT => {
                return Value::Boolean(left.gt(&right));
            }
            AstBinopOp::LE => {
                return Value::Boolean(left.le(&right));
            }
            AstBinopOp::GE => {
                return Value::Boolean(left.ge(&right));
            }
        }
        self.error = "unmatched left and right value types".into();
        return Value::Null();
    }

    fn compute_stack_values(&mut self, op: AstBinopOp) -> Value {
        let mut left = Value::Null();
        let mut right = Value::Null();

        if let Some(value) = self.stack.pop() {
            right = value;
        }
        if let Some(value) = self.stack.pop() {
            left = value;
        }

        return self.compute_values(left, right, op);
    }

    pub fn check_stack_overflow(&mut self) {
        if self.call_stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
        if self.stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
    }

    fn add_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn vmcall(&mut self, index: u8) {
        match index {
            1 => {
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.print(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            2 => {
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.println(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            3 => {
                if let Some(platform) = &self.platform {
                    let string = platform.get_string();
                    let value = Value::String(string.into());
                    self.stack.push(value);
                }
            }
            _ => {
                self.error = format!("vmcall: invalid vmcall index {}", index);
            }
        }
    }

    pub fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Call() => {
                if let Some(value) = self.stack.pop() {
                    if let Value::FunctionRef(addr, args_count) = value {
                        self.call_stack.push(self.pc);
                        self.check_stack_overflow();
                        self.pc = addr;
                        self.pc.sub_low(1);
                        self.add_scope();

                        let diff = self.bfas_stack_end - self.bfas_stack_start;
                        if diff != args_count as i64 {
                            self.error = format!(
                                "call: expected exactly {} arguments, but provided {}",
                                args_count, diff
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
            Opcode::Loadcn(value) => {
                let value = Value::Number(*value);
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::Loadcnu() => {
                self.stack.push(Value::Null());
                self.check_stack_overflow();
            }
            Opcode::Loadcb(flag) => {
                self.stack.push(Value::Boolean(*flag));
                self.check_stack_overflow();
            }
            Opcode::Loadcs(value) => {
                let value = Value::String(value.to_string());
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::Loadv(name) => {
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
                self.error = format!("unknown variable or function: {}", name);
            }
            Opcode::Storev(name) => {
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
            Opcode::Pushret() => {
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
                let value = Value::Array(vec);
                self.stack.push(value);
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

                if let Value::Array(array) = array {
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
                self.error = format!("iafs failed: invalid operand types");
            }
            Opcode::Aiafs(name) => {
                let value;
                let index;
                if let Some(value) = self.stack.pop() {
                    index = value;
                } else {
                    self.error = format!("aiafs failed: no more values on stack for index");
                    return;
                }
                if let Some(some_value) = self.stack.pop() {
                    value = some_value;
                } else {
                    self.error = format!("aiafs failed: no more values on stack for array");
                    return;
                }

                let array;
                if let Some(scope) = self.scopes.last_mut() {
                    if let Some(value) = scope.get_mut(name) {
                        array = value;
                    } else {
                        self.error = format!("aiafs failed: no variable named {}", name);
                        return;
                    }
                } else {
                    self.error = format!("aiafs failed: no scopes");
                    return;
                }

                if let Value::Array(array) = array {
                    if let Value::Number(index) = index {
                        let usize_index = index as usize;
                        if usize_index >= array.len() {
                            self.error = format!(
                                "aiafs failed: index ({}) is larger or equal to array length ({})",
                                usize_index,
                                array.len()
                            );
                            return;
                        }

                        array[usize_index] = value;
                        return;
                    }
                }
                self.error = format!("aiafs failed: invalid operand types");
            }
            Opcode::Bfas() => {
                self.bfas_stack_start = self.stack.len() as i64;
            }
            Opcode::Efas() => {
                self.bfas_stack_end = self.stack.len() as i64;
            }
            Opcode::Pop() => {
                if self.stack.is_empty() {
                    self.error = format!("pop failed: no value in stack");
                } else {
                    self.stack.pop();
                }
            }
            Opcode::Bst(addr) => {
                if let Some(value) = self.stack.pop() {
                    if let Value::Boolean(flag) = value {
                        if flag {
                            self.pc.set_low(*addr - 1);
                        }
                    } else {
                        self.error = "bst failed: value on stack is not a boolean".into();
                    }
                } else {
                    self.error = "bst failed: no value on stack".into();
                }
            }
            Opcode::Br(addr) => {
                self.pc.set_low(*addr - 1);
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
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if !self.error.is_empty() {
            return false;
        }

        let module_index = self.pc.get_high() as usize;
        let opcode_index = self.pc.get_low();
        if module_index >= self.modules.len() {
            return false;
        }

        let opcodes = core::mem::take(&mut self.modules[module_index].opcodes);
        if opcode_index >= opcodes.len() as u32 {
            return false;
        }

        let opcode = &opcodes[opcode_index as usize];

        self.execute_opcode(opcode);

        self.modules[module_index].opcodes = opcodes;

        self.pc.add_low(1);

        return true;
    }
}
