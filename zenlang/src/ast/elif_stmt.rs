use crate::ast::node::Compile;
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstElifStmt {
    pub value: Option<Box<dyn Compile>>,
    pub body: Vec<Box<dyn Compile>>,
}

impl AstElifStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            body: Vec::new(),
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
