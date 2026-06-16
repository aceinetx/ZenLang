use crate::ast::binop::AstBinopOp;
use crate::vm::*;

impl VM {
    pub fn op_add(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::PLUS)?;
        self.stack.push(value);
        Ok(())
    }
    pub fn op_sub(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::MINUS)?;
        self.stack.push(value);
        Ok(())
    }
    pub fn op_mul(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::MUL)?;
        self.stack.push(value);
        Ok(())
    }
    pub fn op_div(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::DIV)?;
        self.stack.push(value);
        Ok(())
    }
}
