use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

pub struct AstDict {
    pub dict: Vec<(String, Box<dyn Compile>)>,
    do_push: bool,
}

impl AstDict {
    pub fn new() -> Self {
        return Self {
            dict: Vec::new(),
            do_push: true,
        };
    }
}

impl Compile for AstDict {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if self.do_push {
            let mut names = Vec::<String>::new();

            for element in self.dict.iter_mut() {
                if let Err(e) = element.1.compile(compiler) {
                    return Err(e);
                }
                names.push(element.0.clone());
            }

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Cdfse(names));
        }

        Ok(())
    }
}
