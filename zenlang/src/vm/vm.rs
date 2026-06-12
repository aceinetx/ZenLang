use crate::module::Module;
use crate::platform::Platform;
use crate::scope::Scope;
use crate::value::*;
use crate::vm::ProgramCounter;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

pub static MAX_STACK_SIZE: usize = 1000;

pub struct VM {
    pub modules: Vec<Module>,
    pub pc: ProgramCounter,
    pub stack: Vec<Value>,
    pub call_stack: Vec<ProgramCounter>,
    pub scopes: Vec<Scope>,
    pub error: String,
    pub ret: Value,
    pub platform: Option<Box<dyn Platform>>,
    pub global_scope: Scope,
    pub halted: bool,
    pub self_var: Value,
    pub args: Vec<Vec<Value>>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            modules: Vec::new(),
            pc: ProgramCounter::new(),
            stack: Vec::new(),
            call_stack: Vec::new(),
            scopes: Vec::new(),
            error: String::new(),
            ret: Value::Null(),
            platform: None,
            global_scope: Scope::new(),
            halted: false,
            self_var: Value::Null(),
            args: Vec::new(),
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
                self.pc.inst = func.addr;
                self.pc.module = self.modules.len() - 1;

                self.add_scope();

                while !self.halted {
                    if !self.step() {
                        break;
                    }
                }

                self.scopes.clear();

                if !self.error.is_empty() {
                    self.halted = true;
                    return Err(format!(
                        "in constructor of module {} at {}: {}",
                        module.name, self.pc, self.error
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
                    self.pc.inst = function.addr;
                    self.pc.module = i;
                    self.add_scope();
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

    pub(crate) fn add_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub(crate) fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn get_function_name_from_pc(&mut self, pc: &ProgramCounter) -> Option<String> {
        if pc.module >= self.modules.len() {
            return None;
        }
        let module = &self.modules[pc.module];
        for function in module.functions.iter().rev() {
            if pc.inst >= function.addr {
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

        if self.pc.module >= self.modules.len() {
            self.error = format!(
                "module pc overflow: {}/{}",
                self.pc.module,
                self.modules.len()
            );
            self.halted = true;
            return false;
        }

        let cycle_module = self.pc.module;
        let opcodes = core::mem::take(&mut self.modules[cycle_module].opcodes);
        if self.pc.inst >= opcodes.len() {
            self.error = format!(
                "inst pc overflow: {}/{} {:?}",
                self.pc.inst,
                opcodes.len(),
                opcodes
            );
            self.halted = true;
            return false;
        }

        let opcode = &opcodes[self.pc.inst as usize];

        self.execute_opcode(opcode);

        self.modules[cycle_module].opcodes = opcodes;

        self.pc.inst = self.pc.inst.wrapping_add(1);

        return true;
    }
}
