use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::vec::*;

pub struct AstNull {
    do_push: bool,
}

impl AstNull {
    pub fn new() -> Self {
        return Self { do_push: true };
    }
}

impl Compile for AstNull {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::Loadcnu());
        }

        Ok(())
    }
}
