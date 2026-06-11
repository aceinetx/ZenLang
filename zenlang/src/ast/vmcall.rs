use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::vec::*;

pub struct AstVmcall {
    pub id: u8,
}

impl AstVmcall {
    pub fn new(id: u8) -> Self {
        return Self { id: id };
    }
}

impl Compile for AstVmcall {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        module.opcodes.push(Opcode::Vmcall(self.id));

        Ok(())
    }
}
