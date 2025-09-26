use crate::ast::node::Compile;
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

pub struct AstElifStmt {
    pub value: Option<Box<dyn Compile>>,
    pub body: Vec<Box<dyn Compile>>,
    pub elif_let: bool,
    pub elif_let_name: String,
    pub elif_let_expr: Option<Box<dyn Compile>>,
}

impl AstElifStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            body: Vec::new(),
            elif_let: false,
            elif_let_name: String::new(),
            elif_let_expr: None,
        };
    }
}

impl Compile for AstElifStmt {
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
