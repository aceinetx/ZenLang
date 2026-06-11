use crate::ast::node::Compile;
use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use alloc::string::*;

pub struct AstGlobalVar {
    pub name: String,
}

impl AstGlobalVar {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
        };
    }
}

impl Compile for AstGlobalVar {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let module = compiler.get_module();
        module.globals.push(self.name.clone());
        Ok(())
    }
}

impl StatementExpression for AstGlobalVar {
    fn disable_push(&mut self) {}
}
