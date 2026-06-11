use crate::ast::block::AstBlock;
use crate::ast::node::Compile;
use crate::compiler::Compiler;
use alloc::string::String;

pub struct AstDefer {
    pub block: AstBlock,
}

impl AstDefer {
    pub fn new() -> Self {
        return Self {
            block: AstBlock::new(),
        };
    }
}

impl Compile for AstDefer {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        self.block.compile(compiler)?;

        Ok(())
    }
}

downcast!(AstDefer);
