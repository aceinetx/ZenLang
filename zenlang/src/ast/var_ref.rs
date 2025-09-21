use crate::{ast::node::Compile, opcode::Opcode};
use alloc::string::*;
use alloc::vec::*;

pub struct AstVarRef {
    pub name: String,
    do_push: bool,
}

impl AstVarRef {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
            do_push: true,
        };
    }
}

impl Compile for AstVarRef {
    fn disable_push(&mut self) {
        self.do_push = false;
    }

    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::Loadv(self.name.clone()));
        }
        Ok(())
    }
}
