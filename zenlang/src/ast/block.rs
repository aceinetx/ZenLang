use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

#[derive(Debug)]
pub struct AstBlock {
    pub children: Vec<Box<dyn Compile>>,
}

impl AstBlock {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
        };
    }
}

impl Compile for AstBlock {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        for block in self.children.iter_mut() {
            block.compile(compiler)?;
        }

        Ok(())
    }
}
