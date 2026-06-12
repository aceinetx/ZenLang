use crate::{ast::node::Compile, compiler::Compiler};
use alloc::string::*;

pub struct AstMod {
    pub name: String,
}

impl AstMod {
    pub fn new(name: String) -> Self {
        return Self { name: name };
    }
}

impl Compile for AstMod {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        module.dependencies.push(self.name.clone());

        Ok(())
    }
}
