use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::boxed::Box;
use alloc::vec::*;

pub struct AstReturn {
    pub value: Box<dyn Compile>,
}

impl AstReturn {
    pub fn new(value: Box<dyn Compile>) -> Self {
        return Self { value: value };
    }
}

impl Compile for AstReturn {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if let Err(e) = self.value.compile(compiler) {
            return Err(e);
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Ret());

        Ok(())
    }
}
