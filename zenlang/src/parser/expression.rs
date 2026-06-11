use crate::ast::array::AstArray;
use crate::ast::array_index::AstArrayIndex;
use crate::ast::binop::{AstBinop, AstBinopOp};
use crate::ast::boolean::AstBoolean;
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
            Token::True => {
                let node = Box::new(AstBoolean::new(true));
                Ok(node)
            }
            Token::False => {
                let node = Box::new(AstBoolean::new(false));
                Ok(node)
            }
            Token::Lbracket => {
                // Array
                let mut node = Box::new(AstArray::new());

                let mut token;
                loop {
                    token = self.next();
                    if matches!(token, Token::Rbracket) {
                        break;
                    }
                    self.back();
                    node.values
                        .push(unwrap_or_ret_error!(self.parse_expression()));

                    token = self.next();
                    match token {
                        Token::Rbracket => break,
                        Token::Comma => continue,
                        _ => panic!("{:?}", token),
                    };
                }

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
                Token::Lbracket => {
                    // Array index
                    let index = Box::new(AstArrayIndex::new(
                        left,
                        unwrap_or_ret_error!(self.parse_expression()),
                    ));

                    let rb = self.next();
                    if !matches!(rb, Token::Rbracket) {
                        return Err(error::Error::ArrayIndexRbracket(rb));
                    }

                    index
                }
                Token::Dot => {
                    // Dotted index
                    let v = self.next();
                    let v: Box<dyn CompileStatementExpression> = match v {
                        Token::Number(num) => Box::new(AstNumber::new(num)),
                        Token::Identifier(ident) => Box::new(AstString::new(ident)),
                        _ => return Err(error::Error::IndexDotSyntax(v)),
                    };

                    let index = Box::new(AstArrayIndex::new(left, v));

                    index
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

    pub(crate) fn parse_bitshift(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_additive());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::BitOperator('>', '>') => AstBinopOp::BITSHR,
                Token::BitOperator('<', '<') => AstBinopOp::BITSHL,
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

    pub(crate) fn parse_equality(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_bitshift());

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

            let right = unwrap_or_ret_error!(self.parse_bitshift());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_bitand(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_equality());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::BitOperator('&', '&') => AstBinopOp::BITAND,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = unwrap_or_ret_error!(self.parse_equality());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_bitor(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let mut left = unwrap_or_ret_error!(self.parse_bitand());

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::BitOperator('|', '|') => AstBinopOp::BITOR,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = unwrap_or_ret_error!(self.parse_bitand());

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_expression(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        return self.parse_bitor();
    }
}
