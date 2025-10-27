use core::cell::RefCell;

use crate::environment;
use crate::environment::Environment;
use crate::module::Module;
use crate::platform::Platform;
use crate::strong_u64::*;
use crate::value::*;
use alloc::boxed::*;
use alloc::format;
use alloc::rc::*;
use alloc::string::*;
use alloc::vec::*;

pub static MAX_STACK_SIZE: usize = 1000;

pub struct VM {
    pub modules: Vec<Module>,
    pub pc: u64,
    pub stack: Vec<Value>,
    pub call_stack: Vec<u64>,
    pub next_environ_index: usize,
    pub environs: Vec<(usize, Environment)>,
    pub environs_stack: Vec<usize>,
    pub error: String,
    pub ret: Value,
    pub platform: Option<Box<dyn Platform>>,
    pub global_scope: Environment,
    pub halted: bool,
    pub self_var: Value,
    pub(crate) bfas_stack_start: Vec<i64>,
    pub(crate) bfas_stack_end: Vec<i64>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            modules: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            next_environ_index: 0,
            environs: Vec::new(),
            environs_stack: Vec::new(),
            error: String::new(),
            ret: Value::Null(),
            platform: None,
            global_scope: Environment::new(),
            halted: false,
            self_var: Value::Null(),
            bfas_stack_start: Vec::new(),
            bfas_stack_end: Vec::new(),
        };
    }

    pub fn load_module(&mut self, module: &Module) -> Result<(), String> {
        // check if already loaded
        for m in self.modules.iter_mut() {
            if m.name == module.name {
                return Ok(());
            }
        }

        self.modules.push(module.clone());

        for var in module.globals.iter() {
            if self.global_scope.get(var).is_some() {
                return Err(format!(
                    "multiple definition of global {} (second definition in module {})",
                    var, module.name
                ));
            }
            self.global_scope.create_if_doesnt_exist(var);
        }

        for func in module.functions.iter() {
            if func.ctor {
                self.pc.set_low(func.addr as u32);
                self.pc.set_high((self.modules.len() - 1) as u32);
                self.push_environment();

                while !self.halted {
                    if !self.step() {
                        break;
                    }
                }

                if !self.error.is_empty() {
                    self.halted = true;
                    return Err(format!(
                        "in constructor of module {}: {}",
                        module.name, self.error
                    ));
                }
                self.halted = false;
            }
        }

        for dependency in module.dependencies.iter() {
            let name = dependency.to_string();

            // check if the dependency is already loaded
            for module in self.modules.iter_mut() {
                if module.name == name {
                    return Ok(());
                }
            }

            // load the dependency
            if let Some(platform) = &self.platform {
                if let Some(module) = platform.get_module(name) {
                    if let Err(e) = self.load_module(&module) {
                        return Err(e);
                    }
                } else {
                    return Err(format!(
                        "unresolved dependency {} (of module {}): not found",
                        dependency, module.name
                    ));
                }
            } else {
                return Err(format!(
                    "unresolved dependency {} (of module {}): self.platform is None",
                    dependency, module.name
                ));
            }
        }
        return Ok(());
    }

    pub fn set_entry_function(&mut self, entry_fn_name: &str) -> Result<(), &'static str> {
        for i in 0..self.modules.len() {
            let module = &self.modules[i];
            for function in module.functions.iter() {
                if function.name == entry_fn_name {
                    self.pc.set_low(function.addr as u32);
                    self.pc.set_high(i as u32);
                    self.push_environment();
                    return Ok(());
                }
            }
        }
        return Err("cannot find entry function");
    }

    pub(crate) fn check_stack_overflow(&mut self) {
        if self.call_stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
        if self.stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
    }

    pub fn push_environment(&mut self) {
        self.environs
            .push((self.next_environ_index, Environment::new()));
        self.environs_stack.push(self.next_environ_index);
        self.next_environ_index += 1;
    }

    pub fn get_environ_by_id(&self, id: usize) -> Option<&Environment> {
        for environ in self.environs.iter() {
            if environ.0 == id {
                return Some(&environ.1);
            }
        }
        return None;
    }

    pub fn get_environ_by_id_mut(&mut self, id: usize) -> Option<&mut Environment> {
        for environ in self.environs.iter_mut() {
            if environ.0 == id {
                return Some(&mut environ.1);
            }
        }
        return None;
    }

    pub fn is_environ_used_in(&self, id: usize, value: &Value) -> bool {
        if let Value::FunctionRefEnv(_, _, i) = value {
            if *i == id {
                return true;
            }
        }

        if let Value::Object(obj) = value {
            match &*obj.borrow() {
                Object::Array(array) => {
                    for value in array.iter() {
                        if self.is_environ_used_in(id, value) {
                            return true;
                        }
                    }
                }
                Object::Dictionary(dict) => {
                    for value in dict.iter() {
                        if self.is_environ_used_in(id, value.1) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn is_environ_used(&self, id: usize) -> bool {
        if id == 0 {
            return true;
        }

        for value in self.stack.iter() {
            if let Value::FunctionRefEnv(_, _, i) = value {
                if *i == id {
                    return true;
                }
            }
        }

        for environ in self.environs.iter() {
            for var in environ.1.vars.iter() {
                if self.is_environ_used_in(id, &var.1) {
                    return true;
                }
            }
        }

        if self.is_environ_used_in(id, &self.ret) {
            return true;
        }

        return false;
    }

    pub fn gc_environs(&mut self) {
        let mut marked: Vec<usize> = Vec::new();
        for i in 0..self.environs.len() {
            let environ = &self.environs[i];

            if !self.is_environ_used(environ.0) {
                marked.push(i);
            }
        }

        for i in marked.iter().rev() {
            self.environs.remove(*i);
        }
    }

    pub fn pop_environment(&mut self) {
        if self.environs_stack.len() == 0 {
            panic!("pop_environment: environs stack is empty");
        }

        self.gc_environs();

        self.environs_stack.pop();
    }

    pub fn get_function_name_from_pc(&mut self, pc: u64) -> Option<String> {
        let module_index = pc.get_high() as usize;
        let opcode_index = pc.get_low();
        if module_index >= self.modules.len() {
            return None;
        }
        let module = &self.modules[module_index];
        for function in module.functions.iter().rev() {
            if opcode_index >= function.addr {
                return Some(function.name.clone());
            }
        }
        return None;
    }

    pub fn run_until_halt(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn step(&mut self) -> bool {
        if self.halted {
            return false;
        }

        let module_index = self.pc.get_high() as usize;
        let opcode_index = self.pc.get_low();
        if module_index >= self.modules.len() {
            self.halted = true;
            return false;
        }

        let opcodes = core::mem::take(&mut self.modules[module_index].opcodes);
        if opcode_index >= opcodes.len() as u32 {
            self.halted = true;
            return false;
        }

        let opcode = &opcodes[opcode_index as usize];

        self.execute_opcode(opcode);
        self.check_stack_overflow();

        self.modules[module_index].opcodes = opcodes;

        self.pc.add_low(1);

        return true;
    }
}
