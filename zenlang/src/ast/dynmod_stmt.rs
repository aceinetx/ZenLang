use crate::ast::node::Compile;
use crate::opcode::*;
use alloc::boxed::*;
use alloc::vec::*;

pub struct AstDynmod {
    pub name: Option<Box<dyn Compile>>,
}

impl AstDynmod {
    pub fn new() -> Self {
        return Self { name: None };
    }
}

impl Compile for AstDynmod {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if let Some(name) = &mut self.name {
            if let Err(e) = name.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("self.name is None".into());
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Vmcall(4));

        Ok(())
    }
}
