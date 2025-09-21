use crate::ast::node::Compile;
use crate::module::Module;
use crate::parser::*;
use alloc::string::*;

pub struct Compiler<'a> {
    parser: &'a mut Parser<'a>,
    module: Module,
}

impl<'a> Compiler<'_> {
    pub fn new(parser: &'a mut Parser<'a>) -> Compiler<'a> {
        let inst = Compiler {
            parser: parser,
            module: Module::new(),
        };

        return inst;
    }

    pub fn get_module(&mut self) -> &mut Module {
        return &mut self.module;
    }

    pub fn compile(&mut self) -> Result<(), String> {
        if let Err(e) = self.parser.parse() {
            return Err(e.into());
        }

        let mut root = core::mem::take(&mut self.parser.root);
        let result = root.compile_all(self);

        self.parser.root = root;

        return result;
    }
}
