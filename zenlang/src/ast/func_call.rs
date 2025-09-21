use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstFuncCall {
    pub reference: Option<Box<dyn Compile>>,
    pub args: Vec<Box<dyn Compile>>,
    do_push: bool,
}

impl AstFuncCall {
    pub fn new() -> Self {
        return Self {
            reference: None,
            args: Vec::new(),
            do_push: true,
        };
    }
}

impl Compile for AstFuncCall {
    fn disable_push(&mut self) {
        self.do_push = false;
    }

    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::Bfas());
        }

        for arg in self.args.iter_mut() {
            if let Err(e) = arg.compile(compiler) {
                return Err(e);
            }
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::Efas());
        }

        {
            if let Some(reference) = &mut self.reference {
                if let Err(e) = reference.compile(compiler) {
                    return Err(e);
                }
            } else {
                return Err("reference is Null".into());
            }
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::Call());
            if self.do_push {
                module.opcodes.push(Opcode::Pushret());
            }
        }

        Ok(())
    }
}
