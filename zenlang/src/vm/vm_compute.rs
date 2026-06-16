use crate::ast::binop::*;
use crate::value::*;
use crate::vm::*;
use alloc::format;
use alloc::string::String;

impl VM {
    pub(crate) fn compute_values(
        &mut self,
        left: &Value,
        right: &Value,
        op: AstBinopOp,
    ) -> Result<Value, VMError> {
        match op {
            AstBinopOp::PLUS => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(left_num + right_num));
                }
                (Value::String(left_str), Value::String(right_str)) => {
                    return Ok(Value::String(format!("{}{}", left_str, right_str)));
                }
                _ => {}
            },
            AstBinopOp::MINUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Ok(Value::Number(left_num - right_num));
                    }
                }
            }
            AstBinopOp::MUL => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(left_num * right_num));
                }
                (Value::String(left_str), Value::Number(right_num)) => {
                    let mut new = String::new();
                    for _ in 0..*right_num as i64 {
                        new.push_str(&left_str);
                    }
                    return Ok(Value::String(new));
                }
                _ => {}
            },
            AstBinopOp::DIV => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        if *right_num == 0.0 {
                            return Err("division by 0".into());
                        }
                        return Ok(Value::Number(left_num / right_num));
                    }
                }
            }
            AstBinopOp::BITSHR => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) >> (*right_num as i64)) as f64,
                    ));
                }
                _ => {}
            },
            AstBinopOp::BITSHL => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) << (*right_num as i64)) as f64,
                    ));
                }
                _ => {}
            },
            AstBinopOp::BITAND => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) & (*right_num as i64)) as f64,
                    ));
                }
                (Value::Boolean(left), Value::Boolean(right)) => {
                    return Ok(Value::Number((*left && *right) as i64 as f64));
                }
                (Value::Number(left_num), Value::Boolean(right_bool)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) & (*right_bool as i64)) as f64,
                    ));
                }
                (Value::Boolean(left_bool), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_bool as i64) & (*right_num as i64)) as f64,
                    ));
                }
                _ => {}
            },
            AstBinopOp::BITOR => match (left, right) {
                (Value::Number(left_num), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) | (*right_num as i64)) as f64,
                    ));
                }
                (Value::Boolean(left), Value::Boolean(right)) => {
                    return Ok(Value::Number((*left || *right) as i64 as f64));
                }
                (Value::Number(left_num), Value::Boolean(right_bool)) => {
                    return Ok(Value::Number(
                        ((*left_num as i64) | (*right_bool as i64)) as f64,
                    ));
                }
                (Value::Boolean(left_bool), Value::Number(right_num)) => {
                    return Ok(Value::Number(
                        ((*left_bool as i64) | (*right_num as i64)) as f64,
                    ));
                }
                _ => {}
            },
            AstBinopOp::EQ => {
                return Ok(Value::Boolean(left.equal(&right, self)));
            }
            AstBinopOp::NEQ => {
                return Ok(Value::Boolean(!left.equal(&right, self)));
            }
            AstBinopOp::LT => {
                return Ok(Value::Boolean(left.lt(&right)));
            }
            AstBinopOp::GT => {
                return Ok(Value::Boolean(left.gt(&right)));
            }
            AstBinopOp::LE => {
                return Ok(Value::Boolean(left.le(&right)));
            }
            AstBinopOp::GE => {
                return Ok(Value::Boolean(left.ge(&right)));
            }
        }

        return Err(format!(
            "unmatched left and right value types: {}, {}",
            left.get_type(),
            right.get_type()
        ));
    }

    pub(crate) fn compute_stack_values(&mut self, op: AstBinopOp) -> Result<Value, VMError> {
        let right = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("compute_stack_values: no value for rhs on stack".into());
            }
        };
        let left = match self.stack.pop() {
            Some(value) => value,
            None => {
                return Err("compute_stack_values: no value for lhs on stack".into());
            }
        };

        return self.compute_values(&left, &right, op);
    }
}
