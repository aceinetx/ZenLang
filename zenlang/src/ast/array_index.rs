use crate::ast::node::StatementExpression;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::vec::*;

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
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if self.do_push {
            if let Err(e) = self.array.compile(compiler) {
                return Err(e);
            }

            if let Err(e) = self.index.compile(compiler) {
                return Err(e);
            }

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
