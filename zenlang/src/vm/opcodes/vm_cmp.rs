use crate::ast::binop::AstBinopOp;
use crate::vm::VM;

impl VM {
    pub fn op_eq(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::EQ);
        self.stack.push(value);
    }

    pub fn op_neq(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::NEQ);
        self.stack.push(value);
    }

    pub fn op_lt(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::LT);
        self.stack.push(value);
    }

    pub fn op_gt(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::GT);
        self.stack.push(value);
    }

    pub fn op_le(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::LE);
        self.stack.push(value);
    }

    pub fn op_ge(&mut self) {
        let value = self.compute_stack_values(AstBinopOp::GE);
        self.stack.push(value);
    }
}
