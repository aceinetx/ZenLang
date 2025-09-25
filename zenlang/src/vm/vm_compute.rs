use crate::ast::binop::*;
use crate::value::*;
use crate::vm::*;

impl<'a> VM<'a> {
    pub(crate) fn compute_values(&mut self, left: Value, right: Value, op: AstBinopOp) -> Value {
        match op {
            AstBinopOp::PLUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num + right_num);
                    }
                }
            }
            AstBinopOp::MINUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num - right_num);
                    }
                }
            }
            AstBinopOp::MUL => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num * right_num);
                    }
                }
            }
            AstBinopOp::DIV => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        if right_num == 0.0 {
                            self.error = "division by 0".into();
                            return Value::Null();
                        }
                        return Value::Number(left_num / right_num);
                    }
                }
            }
            AstBinopOp::BITSHR => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) >> (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::BITSHL => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) << (right_num as i64)) as f64);
                    }
                }
            }
            AstBinopOp::BITAND => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) & (right_num as i64)) as f64);
                    }
                }
                if let Value::Boolean(left) = left {
                    if let Value::Boolean(right) = right {
                        return Value::Number((left && right) as i64 as f64);
                    }
                }
            }
            AstBinopOp::BITOR => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(((left_num as i64) | (right_num as i64)) as f64);
                    }
                }
                if let Value::Boolean(left) = left {
                    if let Value::Boolean(right) = right {
                        return Value::Number((left || right) as i64 as f64);
                    }
                }
            }
            AstBinopOp::EQ => {
                return Value::Boolean(left.equal(&right));
            }
            AstBinopOp::NEQ => {
                return Value::Boolean(!left.equal(&right));
            }
            AstBinopOp::LT => {
                return Value::Boolean(left.lt(&right));
            }
            AstBinopOp::GT => {
                return Value::Boolean(left.gt(&right));
            }
            AstBinopOp::LE => {
                return Value::Boolean(left.le(&right));
            }
            AstBinopOp::GE => {
                return Value::Boolean(left.ge(&right));
            }
        }
        self.error = "unmatched left and right value types".into();
        return Value::Null();
    }

    pub(crate) fn compute_stack_values(&mut self, op: AstBinopOp) -> Value {
        let mut left = Value::Null();
        let mut right = Value::Null();

        if let Some(value) = self.stack.pop() {
            right = value;
        }
        if let Some(value) = self.stack.pop() {
            left = value;
        }

        return self.compute_values(left, right, op);
    }
}
