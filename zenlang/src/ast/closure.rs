use crate::{ast::node::Compile, compiler::Compiler, opcode::Opcode};
use alloc::string::*;
use alloc::vec::*;

pub struct AstClosure {
    pub children: Vec<alloc::boxed::Box<dyn Compile>>,
    pub args: Vec<String>,
}

impl AstClosure {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
            args: Vec::new(),
        };
    }
}

impl Compile for AstClosure {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return Some(&mut self.children);
    }

    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), alloc::string::String> {
        let closure_op_idx;
        {
            let module = compiler.get_module();
            closure_op_idx = module.opcodes.len();
            module
                .opcodes
                .push(Opcode::Closure(0, self.args.len() as u64));
        }

        let module = compiler.get_module();
        for arg in self.args.iter().rev() {
            module.opcodes.push(Opcode::StoreVar(arg.to_string()));
        }

        match self.get_children() {
            Some(children) => {
                for child in children.iter_mut() {
                    if let Err(e) = child.compile_all(compiler) {
                        return Err(e);
                    }
                }
            }
            None => {}
        }

        {
            let module = compiler.get_module();
            if module.opcodes.len() == 0 || !matches!(module.opcodes.last().unwrap(), Opcode::Ret())
            {
                module.opcodes.push(Opcode::LoadNull());
                module.opcodes.push(Opcode::Ret());
            }
        }

        {
            let module = compiler.get_module();
            let len = module.opcodes.len();
            if let Opcode::Closure(addr, _) = &mut module.opcodes[closure_op_idx] {
                *addr = (len - closure_op_idx) as u32 - 1;
            }
        }

        Ok(())
    }
}
