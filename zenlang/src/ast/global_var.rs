use crate::ast::node::Compile;
use crate::ast::node::StatementExpression;
use alloc::string::*;
use alloc::vec::*;

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
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        module.globals.push(self.name.clone());
        Ok(())
    }
}

impl StatementExpression for AstGlobalVar {
    fn disable_push(&mut self) {}
}
