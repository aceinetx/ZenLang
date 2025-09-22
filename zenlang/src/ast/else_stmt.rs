use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

pub struct AstElseStmt {
    pub block: Option<Box<dyn Compile>>,
}

impl AstElseStmt {
    pub fn new() -> Self {
        return Self { block: None };
    }
}

impl Compile for AstElseStmt {
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
