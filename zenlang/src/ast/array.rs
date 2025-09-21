use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstArray {
    pub values: Vec<Box<dyn Compile>>,
    do_push: bool,
}

impl AstArray {
    pub fn new() -> Self {
        return Self {
            values: Vec::new(),
            do_push: true,
        };
    }
}

impl Compile for AstArray {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if self.do_push {
            {
                for value in self.values.iter_mut() {
                    if let Err(e) = value.compile(compiler) {
                        return Err(e);
                    }
                }
            }

            {
                let module = compiler.get_module();
                module.opcodes.push(Opcode::Cafse(self.values.len() as u64));
            }
        }

        Ok(())
    }
}
