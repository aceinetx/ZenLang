use crate::{ast::node::Compile, compiler::Compiler, opcode::Opcode};
use alloc::string::String;

#[derive(Debug)]
pub struct AstAssign {
    pub name: String,
    pub expr: Option<alloc::boxed::Box<dyn Compile>>,
}

impl AstAssign {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
            expr: None,
        };
    }
}

impl Compile for AstAssign {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if let Some(expr) = &mut self.expr {
            expr.compile(compiler)?;
        } else {
            return Err("expr is None".into());
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::StoreVar(self.name.clone()));

        Ok(())
    }
}
