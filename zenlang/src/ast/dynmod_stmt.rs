use crate::ast::node::Compile;
use crate::compiler::Compiler;
use crate::opcode::*;
use alloc::boxed::*;
use alloc::string::String;

pub struct AstDynmod {
    pub name: Box<dyn Compile>,
}

impl AstDynmod {
    pub fn new(name: Box<dyn Compile>) -> Self {
        return Self { name: name };
    }
}

impl Compile for AstDynmod {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        self.name.compile(compiler)?;

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Vmcall(4));

        Ok(())
    }
}
