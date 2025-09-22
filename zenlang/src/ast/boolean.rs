use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::vec::*;

pub struct AstBoolean {
    pub flag: bool,
    do_push: bool,
}

impl AstBoolean {
    pub fn new() -> Self {
        return Self {
            flag: false,
            do_push: true,
        };
    }
}

impl Compile for AstBoolean {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::Loadcb(self.flag));
        }

        Ok(())
    }
}
