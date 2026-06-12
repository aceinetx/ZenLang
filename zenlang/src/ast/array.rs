use crate::ast::node::Compile;
use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::opcode::Opcode;
use alloc::boxed::*;
use alloc::string::String;
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
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if self.do_push {
            for value in self.values.iter_mut() {
                value.compile(compiler)?;
            }

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Cafse(self.values.len() as u64));
        }

        Ok(())
    }
}

impl StatementExpression for AstArray {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
