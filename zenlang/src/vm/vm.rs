use crate::module::Module;
use crate::platform::Platform;
use crate::scope::Scope;
use crate::strong_u64::*;
use crate::value::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

pub static MAX_STACK_SIZE: usize = 1000;

pub struct VM {
    pub modules: Vec<Module>,
    pub owned_modules: Vec<Module>,
    pub pc: u64,
    pub stack: Vec<Value>,
    pub call_stack: Vec<u64>,
    pub scopes: Vec<Scope>,
    pub error: String,
    pub ret: Value,
    pub platform: Option<Box<dyn Platform>>,
    pub gc_current_countdown: usize,
    pub gc_countdown: usize,
    pub gc_enabled: bool,
    pub objects: Vec<(usize, Object)>,
    pub(crate) obj_next_addr: usize,
    pub global_scope: Scope,
    pub halted: bool,
    pub(crate) bfas_stack_start: Vec<i64>,
    pub(crate) bfas_stack_end: Vec<i64>,
}

impl VM {
    pub fn new() -> VM {
        return VM {
            modules: Vec::new(),
            owned_modules: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            scopes: Vec::new(),
            error: String::new(),
            ret: Value::Null(),
            platform: None,
            gc_countdown: 10,
            gc_current_countdown: 10,
            gc_enabled: true,
            objects: Vec::new(),
            obj_next_addr: 0,
            global_scope: Scope::new(),
            halted: false,
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

                while !self.halted {
                    if !self.step() {
                        break;
                    }
                }

                self.scopes.clear();

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

    pub fn get_object(&self, ptr: usize) -> Option<&Object> {
        for pair in self.objects.iter() {
            if pair.0 == ptr {
                return Some(&pair.1);
            }
        }
        return None;
    }

    pub fn get_object_mut(&mut self, ptr: usize) -> Option<&mut Object> {
        for pair in self.objects.iter_mut() {
            if pair.0 == ptr {
                return Some(&mut pair.1);
            }
        }
        return None;
    }

    pub fn add_object(&mut self, obj: Object) -> usize {
        self.objects.push((self.obj_next_addr, obj));
        self.obj_next_addr += 1;
        return self.obj_next_addr - 1;
    }

    pub fn remove_object(&mut self, ptr: usize) {
        if self.get_object(ptr).is_none() {
            self.error = format!("trying to remove an object at 0x{:x} (doesn't exist)", ptr);
            return;
        }
        let mut idx: usize = 0;
        for (i, pair) in self.objects.iter_mut().enumerate() {
            if pair.0 == ptr {
                idx += i;
                break;
            }
        }
        self.objects.remove(idx);
    }

    pub fn step(&mut self) -> bool {
        if self.halted {
            self.gc();
            return false;
        }

        let module_index = self.pc.get_high() as usize;
        let opcode_index = self.pc.get_low();
        if module_index >= self.modules.len() {
            self.halted = true;
            self.gc();
            return false;
        }

        let opcodes = core::mem::take(&mut self.modules[module_index].opcodes);
        if opcode_index >= opcodes.len() as u32 {
            self.halted = true;
            self.gc();
            return false;
        }

        let opcode = &opcodes[opcode_index as usize];

        self.execute_opcode(opcode);

        self.modules[module_index].opcodes = opcodes;

        self.pc.add_low(1);

        if (self.gc_current_countdown == 0 || !self.error.is_empty()) && self.gc_enabled {
            self.gc();
            self.gc_current_countdown = self.objects.len() + 5;
        }
        self.gc_current_countdown -= 1;

        return true;
    }
}
