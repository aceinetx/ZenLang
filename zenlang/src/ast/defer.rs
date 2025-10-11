use crate::ast::node::Compile;
use alloc::boxed::*;
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
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        for block in self.body.iter_mut() {
            if let Err(e) = block.compile(compiler) {
                return Err(e);
            }
        }

        Ok(())
    }
}

downcast!(AstDefer);
