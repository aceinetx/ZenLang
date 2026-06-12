use crate::ast::node::Compile;
use crate::compiler::Compiler;
use crate::opcode::Opcode;
use alloc::boxed::Box;
use alloc::string::String;

pub struct AstReturn {
    pub value: Box<dyn Compile>,
}

impl AstReturn {
    pub fn new(value: Box<dyn Compile>) -> Self {
        return Self { value: value };
    }
}

impl Compile for AstReturn {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        self.value.compile(compiler)?;

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Ret());

        Ok(())
    }
}
