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
        let mut branch_indexes: Vec<usize> = Vec::new();

        // compile expressions
        if let Some(head) = &mut self.head {
            if let Some(value) = &mut head.value {
                if let Err(e) = value.compile(compiler) {
                    return Err(e);
                }
            }

            let module = compiler.get_module();
            branch_indexes.push(module.opcodes.len());

            let opcode = Opcode::Bst(0, 0);
            module.opcodes.push(opcode);
        } else {
            return Err("self.head is None".into());
        }

        // compile blocks
        if let Some(head) = &mut self.head {
            let addr: usize;
            {
                let module = compiler.get_module();
                addr = module.opcodes.len() - 1;
            }

            for node in head.body.iter_mut() {
                if let Err(e) = node.compile_all(compiler) {
                    return Err(e);
                }
            }

            {
                let module = compiler.get_module();
                if let Opcode::Bst(true_addr, _) = &mut module.opcodes[branch_indexes[0]] {
                    *true_addr = addr as u32;
                }
            }
        }

        Ok(())
    }
}
