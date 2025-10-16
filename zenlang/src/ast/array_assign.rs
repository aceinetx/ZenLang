use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

pub struct AstArrayAssign {
    pub name: String,
    pub indexes: Vec<Box<dyn Compile>>,
    pub expr: Option<Box<dyn Compile>>,
}

impl AstArrayAssign {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
            indexes: Vec::new(),
            expr: None,
        };
    }
}

impl Compile for AstArrayAssign {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let len = self.indexes.len();
        for (i, index) in self.indexes.iter_mut().enumerate() {
            if i == len - 1 {
                break;
            }

            if let Err(e) = index.compile(compiler) {
                return Err(e);
            }

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Iafs());
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::LoadVar(self.name.clone()));
        }

        if let Some(expr) = &mut self.expr {
            if let Err(e) = expr.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("expr is None".into());
        }

        if let Err(e) = self.indexes.last_mut().unwrap().compile(compiler) {
            return Err(e);
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Aiafs());

        Ok(())
    }
}
