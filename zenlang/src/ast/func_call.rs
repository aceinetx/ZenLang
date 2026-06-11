use crate::ast::node::StatementExpression;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
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
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::BeginFnArgs());
        }

        for arg in self.args.iter_mut() {
            if let Err(e) = arg.compile(compiler) {
                return Err(e);
            }
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::EndFnArgs());
        }

        {
            if let Err(e) = self.reference.compile(compiler) {
                return Err(e);
            }
        }

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
