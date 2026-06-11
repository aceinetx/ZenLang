use crate::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;
use downcast::*;

/// Trait implemented by all ast nodes
pub trait Compile: Any {
    /// Get the children vector
    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn Compile>>>;

    /// Compiles the current node and it's children recursively
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

    /// Compiles the current node
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String>;
}

pub trait StatementExpression: Any {
    /// Disable pushing for the current node
    ///
    /// This is needed in these cases, where with pushing would just flood the stack with unused data:
    /// ```ignore
    /// fn main {
    ///     123;
    /// }
    /// ```
    /// This would push 123 on the stack with nothing using it, disable_push prevents this
    fn disable_push(&mut self) {}
}

pub trait CompileStatementExpression: Compile + StatementExpression {}

impl<T> CompileStatementExpression for T where T: Compile + StatementExpression {}
