use crate::{ast::node::Compile, opcode::*};
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstWhileStmt {
    pub value: Option<Box<dyn Compile>>,
    pub body: Vec<Box<dyn Compile>>,
}

impl AstWhileStmt {
    pub fn new() -> Self {
        return Self {
            value: None,
            body: Vec::new(),
        };
    }
}

impl Compile for AstWhileStmt {
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let cmp_addr: usize;
        {
            let module = compiler.get_module();
            cmp_addr = module.opcodes.len();
        }

        // * compile expression
        if let Some(value) = &mut self.value {
            if let Err(e) = value.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("self.value is None".into());
        }

        let br_exit_opcode_index;
        {
            let module = compiler.get_module();
            let body_index = module.opcodes.len() + 2;
            module.opcodes.push(Opcode::Bst(body_index as u32));
            br_exit_opcode_index = module.opcodes.len();
            module.opcodes.push(Opcode::Br(0));
        }

        // * compile body
        compiler.while_stmts_break_indexes.push(Vec::new());
        compiler.while_stmts_continue_indexes.push(Vec::new());
        for node in &mut self.body {
            if let Err(e) = node.compile(compiler) {
                return Err(e);
            }
        }

        let exit_addr;
        {
            let module = compiler.get_module();
            exit_addr = module.opcodes.len() + 1;
            if let Opcode::Br(addr) = &mut module.opcodes[br_exit_opcode_index] {
                *addr = exit_addr as u32;
            }

            module.opcodes.push(Opcode::Br(cmp_addr as u32));
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
                    if let Opcode::Br(addr) = &mut module.opcodes[*index] {
                        *addr = exit_addr as u32;
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
                    if let Opcode::Br(addr) = &mut module.opcodes[*index] {
                        *addr = cmp_addr as u32;
                    }
                }
            }

            compiler.while_stmts_continue_indexes.pop();
        }

        Ok(())
    }
}
