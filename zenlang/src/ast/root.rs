use crate::{ast::node::Compile, compiler::Compiler};
use alloc::{string::String, vec::*};

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
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        for child in self.children.iter_mut() {
            child.compile(compiler)?;
        }
        Ok(())
    }
}

impl Default for AstRoot {
    fn default() -> Self {
        return AstRoot::new();
    }
}
