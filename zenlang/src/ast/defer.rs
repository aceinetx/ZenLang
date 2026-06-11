use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

pub struct AstDefer {
    pub body: Vec<Box<dyn Compile>>,
}

impl AstDefer {
    pub fn new() -> Self {
        return Self { body: Vec::new() };
    }
}

impl Compile for AstDefer {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        for block in self.body.iter_mut() {
            block.compile(compiler)?;
        }

        Ok(())
    }
}

downcast!(AstDefer);
