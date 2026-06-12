use crate::compiler::Compiler;
use alloc::string::String;
use core::fmt::Debug;
use downcast::*;

/// Trait implemented by all ast nodes
pub trait Compile: Any + Debug {
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
