use crate::ast::block::AstBlock;
use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::*;

#[derive(Debug)]
pub struct AstIfStmt {
    pub value: Option<Box<dyn Compile>>,
    pub block: AstBlock,
    pub if_let: bool,
    pub if_let_name: String,
    pub if_let_expr: Option<Box<dyn Compile>>,
}

impl AstIfStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            block: AstBlock::new(),
            if_let: false,
            if_let_name: String::new(),
            if_let_expr: None,
        };
    }
}

impl Compile for AstIfStmt {
    fn compile(&mut self, _compiler: &mut Compiler) -> Result<(), String> {
        Ok(())
    }
}
