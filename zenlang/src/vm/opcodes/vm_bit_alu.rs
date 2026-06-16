use crate::ast::binop::AstBinopOp;
use crate::vm::*;

impl VM {
    pub fn op_bshr(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::BITSHR)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_bshl(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::BITSHL)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_band(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::BITAND)?;
        self.stack.push(value);
        Ok(())
    }

    pub fn op_bor(&mut self) -> Result<(), VMError> {
        let value = self.compute_stack_values(AstBinopOp::BITOR)?;
        self.stack.push(value);
        Ok(())
    }
}
