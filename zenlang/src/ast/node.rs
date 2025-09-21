use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

pub trait Compile {
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>>;

    fn disable_push(&mut self) {}

    fn compile_all(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        if let Err(e) = self.compile(compiler) {
            return Err(e);
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
        Ok(())
    }
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String>;
}
