use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::string::*;

pub struct AstGlobalVar {
    pub name: String,
}

impl AstGlobalVar {
    pub fn new(name: String) -> Self {
        return Self { name: name };
    }
}

impl Compile for AstGlobalVar {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        module.globals.push(self.name.clone());
        Ok(())
    }
}
