use crate::ast::node::Compile;
use crate::compiler::Compiler;
use crate::opcode::*;
use alloc::boxed::*;
use alloc::string::String;

pub struct AstDynmod {
    pub name: Option<Box<dyn Compile>>,
}

impl AstDynmod {
    pub fn new() -> Self {
        return Self { name: None };
    }
}

impl Compile for AstDynmod {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if let Some(name) = &mut self.name {
            name.compile(compiler)?;
        } else {
            return Err("self.name is None".into());
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Vmcall(4));

        Ok(())
    }
}
