use crate::ast::binop::AstBinopOp;
use crate::vm::*;

impl VM {
    pub fn op_eq(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::EQ)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_neq(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::NEQ)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_lt(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::LT)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_gt(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::GT)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_le(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::LE)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_ge(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::GE)?;
        self.stack.push(value);
        Ok(())
    }
}
