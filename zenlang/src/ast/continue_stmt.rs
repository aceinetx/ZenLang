use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::vec::*;

pub struct AstContinue {}

impl AstContinue {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Compile for AstContinue {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let addr: usize;
        {
            let module = compiler.get_module();
            addr = module.opcodes.len();
        }

        if let Some(last) = compiler.while_stmts_continue_indexes.last_mut() {
            last.push(addr);
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::Branch(0));
        }

        Ok(())
    }
}
