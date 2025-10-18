use crate::ast::binop::AstBinopOp;
use crate::vm::VM;

impl VM {
    pub fn op_bshr(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::BITSHR);
        self.stack.push(value);
    }

    pub fn op_bshl(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::BITSHL);
        self.stack.push(value);
    }

    pub fn op_band(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::BITAND);
        self.stack.push(value);
    }

    pub fn op_bor(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::BITOR);
        self.stack.push(value);
    }
}
