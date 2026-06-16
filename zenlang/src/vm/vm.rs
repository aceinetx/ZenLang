use core::cell::RefCell;

use crate::module::Module;
use crate::platform::Platform;
use crate::scope::Scope;
use crate::value::*;
use crate::vm::ProgramCounter;
use crate::vm::StopReason;
use crate::vm::global_state::GlobalState;
use alloc::boxed::*;
use alloc::collections::btree_set::BTreeSet;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::*;
use alloc::vec::*;

pub static MAX_STACK_SIZE: usize = 1024;

pub type VMError = String;

pub struct VM {
    pub modules: Vec<Module>,
    pub pc: ProgramCounter,
    pub stack: Vec<Value>,
    pub call_stack: Vec<ProgramCounter>,
    pub scopes: Vec<Scope>,
    pub ret: Value,
    pub platform: Option<Box<dyn Platform>>,
    pub halted: bool,
    pub self_var: Value,
    pub args: Vec<Vec<Value>>,
    pub breakpoints: BTreeSet<ProgramCounter>,
    pub global_state: Rc<RefCell<GlobalState>>,
}

impl VM {
    pub fn new(global_state: Rc<RefCell<GlobalState>>) -> VM {
        return VM {
            modules: Vec::new(),
            pc: ProgramCounter::new(),
            stack: Vec::new(),
            call_stack: Vec::new(),
            scopes: Vec::new(),
            ret: Value::Null(),
            platform: None,
            halted: false,
            self_var: Value::Null(),
            args: Vec::new(),
            breakpoints: BTreeSet::new(),
            global_state: global_state,
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
            let mut state = self.global_state.borrow_mut();

            if state.global_scope.get(var).is_some() {
                return Err(format!(
                    "multiple definition of global {} (second definition in module {})",
                    var, module.name
                ));
            }
            state.global_scope.create_if_doesnt_exist(var);
        }

        for func in module.functions.iter() {
            if func.ctor {
                self.pc.inst = func.addr;
                self.pc.module = self.modules.len() - 1;

                self.add_scope();

                let mut error = Ok(None);
                while !self.halted {
                    error = self.step();
                }

                self.scopes.clear();

                if let Err(e) = error {
                    self.halted = true;
                    return Err(format!(
                        "in constructor of module {} at {}: {}",
                        module.name, self.pc, e
                    ));
                }
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

    pub(crate) fn check_stack_overflow(&mut self) -> Result<(), VMError> {
        if self.call_stack.len() >= MAX_STACK_SIZE {
            return Err("call stack overflow".into());
        }
        if self.stack.len() >= MAX_STACK_SIZE {
            return Err("value stack overflow".into());
        }
        return Ok(());
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

    pub fn step(&mut self) -> Result<Option<StopReason>, VMError> {
        if self.halted {
            return Ok(Some(StopReason::Halt));
        }

        if self.pc.module >= self.modules.len() {
            self.halted = true;
            return Err(format!(
                "module pc overflow: {}/{}",
                self.pc.module,
                self.modules.len()
            ));
        }

        let cycle_module = self.pc.module;
        let opcodes = core::mem::take(&mut self.modules[cycle_module].opcodes);
        if self.pc.inst >= opcodes.len() {
            self.halted = true;
            return Err(format!(
                "inst pc overflow: {}/{} {:?}",
                self.pc.inst,
                opcodes.len(),
                opcodes
            ));
        }

        let opcode = &opcodes[self.pc.inst as usize];

        if let Err(e) = self.execute_opcode(opcode) {
            self.halted = true;
            return Err(e);
        }

        self.modules[cycle_module].opcodes = opcodes;
        self.pc.inst = self.pc.inst.wrapping_add(1);

        if self.breakpoints.contains(&self.pc) {
            return Ok(Some(StopReason::Breakpoint));
        }

        return Ok(None);
    }
}
