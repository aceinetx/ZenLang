use alloc::string::String;

use crate::ast::node::{Compile, StatementExpression};
use crate::compiler::Compiler;
use crate::opcode::Opcode;

pub struct AstNull {
    do_push: bool,
}

impl AstNull {
    pub fn new() -> Self {
        return Self { do_push: true };
    }
}

impl Compile for AstNull {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::LoadNull());
        }

        Ok(())
    }
}

impl StatementExpression for AstNull {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
