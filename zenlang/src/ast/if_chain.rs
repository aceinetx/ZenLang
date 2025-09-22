use crate::ast::elif_stmt::*;
use crate::ast::else_stmt;
use crate::ast::else_stmt::*;
use crate::ast::if_stmt::*;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::vec::*;
use bincode::enc;

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
        let mut end_branch_indexes: Vec<usize> = Vec::new();

        let head: &mut AstIfStmt;
        let else_addr: usize;

        if let Some(h) = &mut self.head {
            head = h;
        } else {
            return Err("self.head is None".into());
        }

        // compile expressions
        // * head
        {
            if let Some(value) = &mut head.value {
                if let Err(e) = value.compile(compiler) {
                    return Err(e);
                }
            }

            let module = compiler.get_module();
            branch_indexes.push(module.opcodes.len());

            let opcode = Opcode::Bst(0);
            module.opcodes.push(opcode);
        }

        // * elifs
        for elif_node in self.elifs.iter_mut() {
            if let Some(value) = &mut elif_node.value {
                if let Err(e) = value.compile(compiler) {
                    return Err(e);
                }
            }

            let module = compiler.get_module();
            branch_indexes.push(module.opcodes.len());

            let opcode = Opcode::Bst(0);
            module.opcodes.push(opcode);
        }

        {
            let module = compiler.get_module();
            branch_indexes.push(module.opcodes.len());

            let opcode = Opcode::Br(0);
            module.opcodes.push(opcode);
        }

        // compile blocks
        // * head
        {
            let addr: usize;
            {
                let module = compiler.get_module();
                addr = module.opcodes.len();
            }

            for node in head.body.iter_mut() {
                if let Err(e) = node.compile_all(compiler) {
                    return Err(e);
                }
            }

            {
                let module = compiler.get_module();
                end_branch_indexes.push(module.opcodes.len());

                let opcode = Opcode::Br(0);
                module.opcodes.push(opcode);
            }

            {
                let module = compiler.get_module();
                if let Opcode::Bst(bst_addr) = &mut module.opcodes[branch_indexes[0]] {
                    *bst_addr = addr as u32;
                }
            }
        }

        // * elifs
        for i in 0..self.elifs.len() {
            let elif = &mut self.elifs[i];

            let addr: usize;
            {
                let module = compiler.get_module();
                addr = module.opcodes.len();
            }

            for node in elif.body.iter_mut() {
                if let Err(e) = node.compile_all(compiler) {
                    return Err(e);
                }
            }

            {
                let module = compiler.get_module();
                end_branch_indexes.push(module.opcodes.len());

                let opcode = Opcode::Br(0);
                module.opcodes.push(opcode);
            }

            {
                let module = compiler.get_module();
                if let Opcode::Bst(bst_addr) = &mut module.opcodes[branch_indexes[1 + i]] {
                    *bst_addr = addr as u32;
                }
            }
        }

        // * else
        {
            let module = compiler.get_module();
            else_addr = module.opcodes.len();
        }
        if let Some(else_stmt) = &mut self.else_node {
            for node in else_stmt.body.iter_mut() {
                if let Err(e) = node.compile_all(compiler) {
                    return Err(e);
                }
            }
        }

        {
            let module = compiler.get_module();
            let opcode = &mut module.opcodes[*branch_indexes.last().unwrap()];
            if let Opcode::Br(addr) = opcode {
                *addr = else_addr as u32;
            }
        }

        // * set end branch indexes
        let len: usize;
        {
            let module = compiler.get_module();
            len = module.opcodes.len();
        }
        for index in end_branch_indexes.iter() {
            let module = compiler.get_module();
            let opcode = &mut module.opcodes[*index];
            if let Opcode::Br(addr) = opcode {
                *addr = len as u32;
            }
        }

        Ok(())
    }
}
