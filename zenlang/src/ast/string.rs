use crate::ast::node::Compile;
use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::opcode::Opcode;
use alloc::string::*;

pub struct AstString {
    pub string: String,
    do_push: bool,
}

impl AstString {
    pub fn new(string: String) -> Self {
        return Self {
            string: string,
            do_push: true,
        };
    }
}

impl Compile for AstString {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        if self.do_push {
            module
                .opcodes
                .push(Opcode::LoadStr(self.string.to_string()));
        }

        Ok(())
    }
}

impl StatementExpression for AstString {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
