use crate::ast::block::AstBlock;
use crate::compiler::Compiler;
use crate::{ast::node::Compile, opcode::*};
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

#[derive(Debug)]
pub struct AstWhileStmt {
    pub value: Option<Box<dyn Compile>>,
    pub body: AstBlock,
}

impl AstWhileStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            body: AstBlock::new(),
        };
    }
}

impl Compile for AstWhileStmt {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let cmp_addr: usize;
        {
            let module = compiler.get_module();
            cmp_addr = module.opcodes.len();
        }

        // * compile expression
        if let Some(value) = &mut self.value {
            value.compile(compiler)?;
        } else {
            return Err("self.value is None".into());
        }

        let br_exit_opcode_index;
        {
            let module = compiler.get_module();
            let body_index = module.opcodes.len() + 2;
            module.opcodes.push(Opcode::BranchTrue(body_index));
            br_exit_opcode_index = module.opcodes.len();
            module.opcodes.push(Opcode::Branch(0));
        }

        // * compile body
        compiler.while_stmts_break_indexes.push(Vec::new());
        compiler.while_stmts_continue_indexes.push(Vec::new());
        self.body.compile(compiler)?;

        let exit_addr;
        {
            let module = compiler.get_module();
            exit_addr = module.opcodes.len() + 1;
            if let Opcode::Branch(addr) = &mut module.opcodes[br_exit_opcode_index] {
                *addr = exit_addr;
            }

            module.opcodes.push(Opcode::Branch(cmp_addr));
        }

        // break statementss
        {
            let last: Vec<usize>;
            {
                last = compiler.while_stmts_break_indexes.last().unwrap().clone();
            }

            {
                let module = compiler.get_module();
                for index in last.iter() {
                    if let Opcode::Branch(addr) = &mut module.opcodes[*index] {
                        *addr = exit_addr;
                    }
                }
            }

            compiler.while_stmts_break_indexes.pop();
        }

        // continue statementss
        {
            let last: Vec<usize>;
            {
                last = compiler
                    .while_stmts_continue_indexes
                    .last()
                    .unwrap()
                    .clone();
            }

            {
                let module = compiler.get_module();
                for index in last.iter() {
                    if let Opcode::Branch(addr) = &mut module.opcodes[*index] {
                        *addr = cmp_addr;
                    }
                }
            }

            compiler.while_stmts_continue_indexes.pop();
        }

        Ok(())
    }
}
