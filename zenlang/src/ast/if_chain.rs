use crate::ast::elif_stmt::*;
use crate::ast::else_stmt::*;
use crate::ast::if_stmt::*;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

pub struct AstIfChain {
    pub head: Option<AstIfStmt>,
    pub elifs: Vec<AstElifStmt>,
    pub else_node: Option<AstElseStmt>,
}

impl AstIfChain {
    pub fn new() -> Self {
        return Self {
            head: None,
            elifs: Vec::new(),
            else_node: None,
        };
    }
}

impl Compile for AstIfChain {
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();

        let head_bst_opcode_index = module.opcodes.len();
        let else_index: usize;

        if let Some(head) = &mut self.head {
        } else {
            return Err("self.head is None".into());
        }

        if let Some(else_node) = &mut self.else_node {
            else_index = module.opcodes.len();
        } else {
            return Err("self.else_node is None".into());
        }

        Ok(())
    }
}
