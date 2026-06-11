use crate::ast::binop::{AstBinop, AstBinopOp};
use crate::ast::func_call::AstFuncCall;
use crate::ast::node::{Compile, CompileStatementExpression};
use crate::ast::null::AstNull;
use crate::ast::number::AstNumber;
use crate::ast::string::AstString;
use crate::ast::var_ref::AstVarRef;
use crate::parser::unwrap_or_ret_error;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::Box;

impl Parser<'_> {
    pub(crate) fn parse_primary(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let token = self.next();
        match token {
            Token::Identifier(ident) => {
                let node = Box::new(AstVarRef::new(ident));

                if matches!(self.next(), Token::Lparen) {
                    // Function call
                }
                self.back();

                Ok(node)
            }
            Token::Number(number) => {
                let node = Box::new(AstNumber::new(number));
                Ok(node)
            }
            Token::String(str) => {
                let node = Box::new(AstString::new(str));
                Ok(node)
            }
            Token::Null => {
                let node = Box::new(AstNull::new());
                Ok(node)
            }
            _ => panic!("{:?}", token),
        }
    }

    pub(crate) fn parse_postfix(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_primary());

        let mut token;
        loop {
            token = self.next();
            left = match token {
                Token::Lparen => {
                    // Function call
                    let mut func_call = Box::new(AstFuncCall::new(left));
                    loop {
                        if matches!(self.next(), Token::Rparen) {
                            break;
                        }

                        self.back();
                        let expr = unwrap_or_ret_error!(self.parse_expression());
                        func_call.args.push(expr);

                        self.back();
                        let comma = self.next();
                        if !matches!(comma, Token::Comma) {
                            return Err(error::Error::FunccallExpectedComma(comma));
                        }
                    }

                    func_call
                }
                _ => {
                    self.back();
                    break;
                }
            };
        }
        Ok(left)
    }

    pub(crate) fn parse_multiplicative(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_postfix());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::Operator('*') => AstBinopOp::MUL,
                Token::Operator('/') => AstBinopOp::DIV,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = unwrap_or_ret_error!(self.parse_postfix());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_additive(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_multiplicative());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::Operator('+') => AstBinopOp::PLUS,
                Token::Operator('-') => AstBinopOp::MINUS,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = unwrap_or_ret_error!(self.parse_multiplicative());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_equality(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_additive());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::OperatorCmp('=', '=') => AstBinopOp::EQ,
                Token::OperatorCmp('!', '=') => AstBinopOp::NEQ,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = unwrap_or_ret_error!(self.parse_additive());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_expression(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        return self.parse_equality();
    }
}
