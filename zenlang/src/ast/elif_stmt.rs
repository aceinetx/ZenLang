use crate::ast::block::AstBlock;
use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::*;

pub struct AstElifStmt {
    pub value: Option<Box<dyn Compile>>,
    pub block: AstBlock,
    pub elif_let: bool,
    pub elif_let_name: String,
    pub elif_let_expr: Option<Box<dyn Compile>>,
}

impl AstElifStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            block: AstBlock::new(),
            elif_let: false,
            elif_let_name: String::new(),
            elif_let_expr: None,
        };
    }
}

impl Compile for AstElifStmt {
    fn compile(&mut self, _compiler: &mut Compiler) -> Result<(), String> {
        Ok(())
    }
}
