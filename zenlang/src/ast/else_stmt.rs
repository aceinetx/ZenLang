use crate::ast::block::AstBlock;
use crate::ast::node::Compile;
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstElseStmt {
    pub body: AstBlock,
}

impl AstElseStmt {
    pub fn new() -> Self {
        return Self {
            body: AstBlock::new(),
        };
    }
}

impl Compile for AstElseStmt {
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        _compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        Ok(())
    }
}
