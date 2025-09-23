use crate::module::ModuleFunction;
use crate::{ast::node::Compile, compiler::Compiler, opcode::Opcode};
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

pub struct AstFunction {
    pub children: Vec<alloc::boxed::Box<dyn Compile>>,
    pub name: String,
    pub args: Vec<String>,
}

impl AstFunction {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
            name: String::new(),
            args: Vec::new(),
        };
    }
}

impl Compile for AstFunction {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return Some(&mut self.children);
    }

    fn compile_all(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if let Err(e) = self.compile(compiler) {
            return Err(e);
        }

        match self.get_children() {
            Some(children) => {
                for child in children.iter_mut() {
                    if let Err(e) = child.compile_all(compiler) {
                        return Err(e);
                    }
                }
            }
            None => {}
        }

        let module = compiler.get_module();
        if module.opcodes.len() == 0 || !matches!(module.opcodes.last().unwrap(), Opcode::Ret()) {
            module.opcodes.push(Opcode::Loadcnu());
            module.opcodes.push(Opcode::Ret());

            compiler
                .warnings
                .push(format!("function {} implicitly returns null", self.name));
        }

        Ok(())
    }

    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), alloc::string::String> {
        if self.name == "main" && self.args.len() > 0 {
            return Err("main function should not accept any arguments".into());
        }

        let module = compiler.get_module();
        {
            let name = self.name.to_string();
            module.functions.push(ModuleFunction::new(
                name,
                module.opcodes.len() as u32,
                self.args.len() as u64,
            ));
        }

        for arg in self.args.iter().rev() {
            module.opcodes.push(Opcode::Storev(arg.to_string()));
        }

        Ok(())
    }
}
