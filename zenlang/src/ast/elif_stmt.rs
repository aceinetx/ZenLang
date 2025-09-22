use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

pub struct AstElifStmt {
    pub value: Option<Box<dyn Compile>>,
    pub block: Option<Box<dyn Compile>>,
}

impl AstElifStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            block: None,
        };
    }
}

impl Compile for AstElifStmt {
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();

        Ok(())
    }
}
