use crate::ast::elif_stmt::*;
use crate::ast::else_stmt::*;
use crate::ast::if_stmt::*;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
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
        // To the developer who is going to read this code:
        //
        // Dont. Don't make a mistake by trying to understand what it does
        // Even I (aceinetx) can't really understand it
        // This code literally works on magic

        // We have this code:
        /*
           if x == 2 {
               ...
           } elif x == 1 {
               ...
           } else {
               ...
           }
        */
        // Our goal is to compile it to this: (Imagine we have labels)
        /*
           Loadv("x")
           Loadcn(2.0)
           Bst("1")
           Loadv("x")
           Loadcn(1.0)
           Bst("2")
           Br("3")
        1: ... (head (if))
           Br("4")
        2: ... (elif)
           Br("4")
        3: ... (else)
        4: the rest of the code
        */

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
            if !head.if_let {
                // if
                if let Some(value) = &mut head.value {
                    if let Err(e) = value.compile(compiler) {
                        return Err(e);
                    }
                } else {
                    return Err("head.value is None".into());
                }

                let module = compiler.get_module();
                branch_indexes.push(module.opcodes.len());

                let opcode = Opcode::Bst(0);
                module.opcodes.push(opcode);
            } else {
                // if let
                if let Some(value) = &mut head.if_let_expr {
                    if let Err(e) = value.compile(compiler) {
                        return Err(e);
                    }
                } else {
                    return Err("head.if_let_expr is None".into());
                }

                let module = compiler.get_module();
                module
                    .opcodes
                    .push(Opcode::Storev(head.if_let_name.clone()));
                module.opcodes.push(Opcode::Loadv(head.if_let_name.clone()));

                branch_indexes.push(module.opcodes.len());
                module.opcodes.push(Opcode::Bsnn(0));
            }
        }

        // * elifs
        for elif_node in self.elifs.iter_mut() {
            if !elif_node.elif_let {
                // elif
                if let Some(value) = &mut elif_node.value {
                    if let Err(e) = value.compile(compiler) {
                        return Err(e);
                    }
                }

                let module = compiler.get_module();
                branch_indexes.push(module.opcodes.len());

                let opcode = Opcode::Bst(0);
                module.opcodes.push(opcode);
            } else {
                // elif let
                if let Some(value) = &mut elif_node.elif_let_expr {
                    if let Err(e) = value.compile(compiler) {
                        return Err(e);
                    }
                } else {
                    return Err("elif_node.elif_let_expr is None".into());
                }

                let module = compiler.get_module();
                module
                    .opcodes
                    .push(Opcode::Storev(elif_node.elif_let_name.clone()));
                module
                    .opcodes
                    .push(Opcode::Loadv(elif_node.elif_let_name.clone()));

                branch_indexes.push(module.opcodes.len());
                module.opcodes.push(Opcode::Bsnn(0));
            }
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
                // borrow checker workaround
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
                if let Opcode::Bsnn(bsnn_addr) = &mut module.opcodes[branch_indexes[0]] {
                    *bsnn_addr = addr as u32;
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
                if let Opcode::Bsnn(bsnn_addr) = &mut module.opcodes[branch_indexes[1 + i]] {
                    *bsnn_addr = addr as u32;
                }
            }
        }

        // * else
        {
            // borrow checker workaround
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
            // Set the last branch opcode (the one before the blocks opcodes) to the else_addr, which is:
            // - The end of the chain if no else is present
            // - The start of else if else if present
            // ? I'm not sure about this, but this made every test pass
            let module = compiler.get_module();
            let opcode = &mut module.opcodes[*branch_indexes.last().unwrap()];
            if let Opcode::Br(addr) = opcode {
                *addr = else_addr as u32;
            }
        }

        // * set end branch indexes
        // End branch indexes are in every block, how if statements work is that we jump to the end, skipping every
        // elif and else. We just set every opcode in the block to the end.
        let len: usize;
        {
            // borrow checker workaround
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
