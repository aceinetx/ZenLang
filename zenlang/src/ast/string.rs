use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::string::*;
use alloc::vec::*;

pub struct AstString {
    pub string: String,
    do_push: bool,
}

impl AstString {
    pub fn new() -> Self {
        return Self {
            string: String::new(),
            do_push: true,
        };
    }
}

impl Compile for AstString {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::LoadStr(self.string.to_string()));
        }

        Ok(())
    }
}
