use alloc::{string::String, vec::Vec};

use crate::{
    ast::{
        block::AstBlock,
        node::{Compile, StatementExpression},
    },
    compiler::Compiler,
    opcode::Opcode,
};

#[derive(Debug)]
pub struct AstLambda {
    do_push: bool,
    pub block: AstBlock,
    pub args: Vec<String>,
}

impl AstLambda {
    pub fn new() -> Self {
        return Self {
            do_push: true,
            block: AstBlock::new(),
            args: Vec::new(),
        };
    }
}

impl Compile for AstLambda {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let branch_opcode_index;
        {
            let module = compiler.get_module();
            branch_opcode_index = module.opcodes.len();
            module.opcodes.push(Opcode::Branch(0));

            for arg in self.args.iter().rev() {
                module.opcodes.push(Opcode::StoreArg(arg.clone()));
            }
        }

        self.block.compile(compiler)?;

        {
            let module = compiler.get_module();
            let len = module.opcodes.len();
            if let Opcode::Branch(addr) = &mut module.opcodes[branch_opcode_index] {
                *addr = len;
            }

            module
                .opcodes
                .push(Opcode::Lambda(branch_opcode_index + 1, self.args.len()))
        }

        Ok(())
    }
}

impl StatementExpression for AstLambda {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
