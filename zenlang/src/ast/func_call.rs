use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

pub struct AstFuncCall {
    pub reference: Box<dyn Compile>,
    pub args: Vec<Box<dyn Compile>>,
    do_push: bool,
}

impl AstFuncCall {
    pub fn new(reference: Box<dyn Compile>) -> Self {
        return Self {
            reference: reference,
            args: Vec::new(),
            do_push: true,
        };
    }
}

impl Compile for AstFuncCall {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::BeginFnArgs());
        }

        for arg in self.args.iter_mut() {
            arg.compile(compiler)?;
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::EndFnArgs());
        }

        self.reference.compile(compiler)?;

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::Call());
            if self.do_push {
                module.opcodes.push(Opcode::PushRet());
            }
        }

        Ok(())
    }
}

impl StatementExpression for AstFuncCall {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
