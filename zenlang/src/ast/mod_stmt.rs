use crate::ast::node::Compile;
use alloc::string::*;
use alloc::vec::*;

pub struct AstMod {
    pub name: String,
}

impl AstMod {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
        };
    }
}

impl Compile for AstMod {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        module.dependencies.push(self.name.clone());

        Ok(())
    }
}
