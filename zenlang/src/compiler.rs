use crate::ast::node::Compile;
use crate::module::Module;
use crate::parser::parser::*;
use alloc::string::*;
use alloc::vec::*;

pub struct Compiler<'a> {
    parser: &'a mut Parser<'a>,
    module: Module,
    pub while_stmts_break_indexes: Vec<Vec<usize>>,
    pub while_stmts_continue_indexes: Vec<Vec<usize>>,
    pub warnings: Vec<String>,
}

impl<'a> Compiler<'_> {
    pub fn new(parser: &'a mut Parser<'a>) -> Compiler<'a> {
        let inst = Compiler {
            parser: parser,
            module: Module::new(),
            while_stmts_break_indexes: Vec::new(),
            while_stmts_continue_indexes: Vec::new(),
            warnings: Vec::new(),
        };

        return inst;
    }

    pub fn get_module(&mut self) -> &mut Module {
        return &mut self.module;
    }

    pub fn compile(&mut self) -> Result<(), String> {
        self.warnings.clear();
        if let Err(e) = self.parser.parse() {
            return Err(e.into());
        }

        let mut root = core::mem::take(&mut self.parser.root);
        let result = root.compile_all(self);

        self.parser.root = root;

        return result;
    }
}
