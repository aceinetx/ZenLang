use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::string::*;

pub struct AstVarRef {
    pub name: String,
    do_push: bool,
}

impl AstVarRef {
    pub fn new(name: String) -> Self {
        return Self {
            name: name,
            do_push: true,
        };
    }
}

impl Compile for AstVarRef {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::LoadVar(self.name.clone()));
        }
        Ok(())
    }
}

impl StatementExpression for AstVarRef {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
