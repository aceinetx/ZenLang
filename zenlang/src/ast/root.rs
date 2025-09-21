use crate::ast::node::Compile;
use alloc::vec::*;

pub struct AstRoot {
    pub children: Vec<alloc::boxed::Box<dyn Compile>>,
}

impl AstRoot {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }
}

impl Compile for AstRoot {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return Some(&mut self.children);
    }

    fn compile(
        &mut self,
        _compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        Ok(())
    }
}

impl Default for AstRoot {
    fn default() -> Self {
        return AstRoot::new();
    }
}
