use crate::ast::node::Compile;
use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
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
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if self.do_push {
            let mut names = Vec::<String>::new();

            for element in self.dict.iter_mut() {
                element.1.compile(compiler)?;
                names.push(element.0.clone());
            }

            let module = compiler.get_module();
            module.opcodes.push(Opcode::Cdfse(names));
        }

        Ok(())
    }
}

impl StatementExpression for AstDict {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
