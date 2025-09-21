use crate::module::ModuleFunction;
use crate::{ast::node::Compile, opcode::Opcode};
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

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        {
            let name = self.name.to_string();
            module.functions.push(ModuleFunction::new(
                name,
                module.opcodes.len() as u32,
                self.args.len() as u64,
            ));
        }
        //module.opcodes.push(Opcode::Scopenew());

        for arg in self.args.iter().rev() {
            module.opcodes.push(Opcode::Storev(arg.to_string()));
        }

        Ok(())
    }
}
