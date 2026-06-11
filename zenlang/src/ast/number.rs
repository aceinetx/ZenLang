use crate::ast::node::{Compile, StatementExpression};
use crate::opcode::Opcode;
use alloc::vec::*;

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
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
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
