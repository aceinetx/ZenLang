use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::String;

#[derive(Debug)]
pub struct AstArrayIndex {
    pub array: Box<dyn Compile>,
    pub index: Box<dyn Compile>,
    do_push: bool,
}

impl AstArrayIndex {
    pub fn new(array: Box<dyn Compile>, index: Box<dyn Compile>) -> Self {
        return Self {
            array: array,
            index: index,
            do_push: true,
        };
    }
}

impl Compile for AstArrayIndex {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if self.do_push {
            self.array.compile(compiler)?;
            self.index.compile(compiler)?;

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Iafs());
        }
        Ok(())
    }
}

impl StatementExpression for AstArrayIndex {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
