use alloc::string::String;

use crate::ast::node::Compile;
use crate::compiler::Compiler;
use crate::opcode::Opcode;

pub struct AstVmcall {
    pub id: u8,
}

impl AstVmcall {
    pub fn new(id: u8) -> Self {
        return Self { id: id };
    }
}

impl Compile for AstVmcall {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        module.opcodes.push(Opcode::Vmcall(self.id));

        Ok(())
    }
}
