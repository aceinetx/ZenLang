use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstArrayIndex {
    pub array: Option<Box<dyn Compile>>,
    pub index: Option<Box<dyn Compile>>,
    do_push: bool,
}

impl AstArrayIndex {
    pub fn new() -> Self {
        return Self {
            array: None,
            index: None,
            do_push: true,
        };
    }
}

impl Compile for AstArrayIndex {
    fn disable_push(&mut self) {
        self.do_push = false;
    }

    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if self.do_push {
            if let Some(array) = &mut self.array {
                if let Err(e) = array.compile(compiler) {
                    return Err(e);
                }
            } else {
                return Err("array is None".into());
            }
            if let Some(index) = &mut self.index {
                if let Err(e) = index.compile(compiler) {
                    return Err(e);
                }
            } else {
                return Err("index is None".into());
            }

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Iafs());
        }
        Ok(())
    }
}
