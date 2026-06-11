use alloc::string::String;

use crate::ast::node::Compile;
use crate::compiler::Compiler;
use crate::opcode::Opcode;

pub struct AstContinue {}

impl AstContinue {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Compile for AstContinue {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
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
