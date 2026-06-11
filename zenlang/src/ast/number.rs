use alloc::string::String;

use crate::ast::node::{Compile, StatementExpression};
use crate::compiler::Compiler;
use crate::opcode::Opcode;

pub struct AstNumber {
    pub number: f64,
    do_push: bool,
}

impl AstNumber {
    pub fn new(number: f64) -> Self {
        return Self {
            number: number,
            do_push: true,
        };
    }
}

impl Compile for AstNumber {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::LoadConstant(self.number));
        }

        Ok(())
    }
}

impl StatementExpression for AstNumber {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
