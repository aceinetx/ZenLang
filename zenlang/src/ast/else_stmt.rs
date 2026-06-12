use alloc::string::String;

use crate::ast::block::AstBlock;
use crate::ast::node::Compile;
use crate::compiler::Compiler;

#[derive(Debug)]
pub struct AstElseStmt {
    pub block: AstBlock,
}

impl AstElseStmt {
    pub fn new() -> Self {
        return Self {
            block: AstBlock::new(),
        };
    }
}

impl Compile for AstElseStmt {
    fn compile(&mut self, _compiler: &mut Compiler) -> Result<(), String> {
        Ok(())
    }
}
