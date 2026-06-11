use crate::compiler::Compiler;
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
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        {
            let module = compiler.get_module();
            module.opcodes.push(Opcode::LoadVar(self.name.clone()));
        }

        let len = self.indexes.len();
        for (i, index) in self.indexes.iter_mut().enumerate() {
            if i == len - 1 {
                break;
            }

            index.compile(compiler)?;

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Iafs());
        }

        if let Some(expr) = &mut self.expr {
            expr.compile(compiler)?;
        } else {
            return Err("expr is None".into());
        }

        self.indexes.last_mut().unwrap().compile(compiler)?;

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Aiafs());

        Ok(())
    }
}
