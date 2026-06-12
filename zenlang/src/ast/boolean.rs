use alloc::string::String;

use crate::ast::node::{Compile, StatementExpression};
use crate::compiler::Compiler;
use crate::opcode::Opcode;

#[derive(Debug)]
pub struct AstBoolean {
    pub flag: bool,
    do_push: bool,
}

impl AstBoolean {
    pub fn new(flag: bool) -> Self {
        return Self {
            flag: flag,
            do_push: true,
        };
    }
}

impl Compile for AstBoolean {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::LoadBool(self.flag));
        }

        Ok(())
    }
}

impl StatementExpression for AstBoolean {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
