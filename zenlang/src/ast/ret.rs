use crate::ast::node::Compile;
use crate::opcode::Opcode;
use alloc::vec::*;

pub struct AstReturn {
    pub value: Option<alloc::boxed::Box<dyn Compile>>,
}

impl AstReturn {
    pub fn new() -> Self {
        return Self { value: None };
    }
}

impl Compile for AstReturn {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        match &mut self.value {
            None => {
                return Err("self.value is None".into());
            }
            Some(value) => {
                if let Err(e) = value.compile(compiler) {
                    return Err(e);
                }

                {
                    let module = compiler.get_module();
                    module.opcodes.push(Opcode::Ret());
                }
            }
        }

        Ok(())
    }
}
