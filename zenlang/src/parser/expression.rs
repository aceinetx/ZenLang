use crate::ast::array::AstArray;
use crate::ast::array_index::AstArrayIndex;
use crate::ast::binop::{AstBinop, AstBinopOp};
use crate::ast::boolean::AstBoolean;
use crate::ast::dict::AstDict;
use crate::ast::func_call::AstFuncCall;
use crate::ast::lambda::AstLambda;
use crate::ast::node::CompileStatementExpression;
use crate::ast::null::AstNull;
use crate::ast::number::AstNumber;
use crate::ast::string::AstString;
use crate::ast::var_ref::AstVarRef;
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
                    node.values.push(self.parse_expression()?);

                    token = self.next();
                    match token {
                        Token::Rbracket => break,
                        Token::Comma => continue,
                        _ => panic!("{:?}", token),
                    };
                }

                Ok(node)
            }
            Token::Lbrace => {
                // Dictionary
                let mut node = Box::new(AstDict::new());

                let mut token;
                loop {
                    token = self.next();
                    if matches!(token, Token::Rbrace) {
                        break;
                    }

                    let key = match token {
                        Token::String(key) => key,
                        _ => return Err(error::Error::DictStrKey(token)),
                    };

                    let eq = self.next();
                    if !matches!(eq, Token::Assign) {
                        return Err(error::Error::DictEqual(eq));
                    }

                    let value = self.parse_expression()?;

                    node.dict.push((key, value));

                    token = self.next();
                    match token {
                        Token::Rbrace => break,
                        Token::Comma => continue,
                        _ => panic!("{:?}", token),
                    };
                }

                Ok(node)
            }
            Token::Fn => {
                let mut lambda = AstLambda::new();

                let mut token;
                loop {
                    token = self.next();
                    let name = match token {
                        Token::Lbrace => {
                            self.back();
                            break;
                        }
                        Token::Identifier(ident) => ident,
                        _ => return Err(error::Error::LambdaArgIdent(token)),
                    };

                    lambda.args.push(name);
                }

                lambda.block = self.parse_block()?;

                Ok(Box::new(lambda))
            }
            Token::Lparen => {
                let expr = self.parse_expression()?;
                let rp = self.next();
                if !matches!(rp, Token::Rparen) {
                    return Err(error::Error::ExprRparen(rp));
                }

                Ok(expr)
            }
            _ => return Err(error::Error::ExprUnexpectedPrimary(token)),
        }
    }

    pub(crate) fn parse_postfix(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_primary()?;

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
                        let expr = self.parse_expression()?;
                        func_call.args.push(expr);

                        let comma = self.next();
                        match comma {
                            Token::Comma => (),
                            Token::Rparen => self.back(),
                            _ => return Err(error::Error::FunccallExpectedComma(comma)),
                        }
                    }

                    func_call
                }
                Token::Lbracket => {
                    // Array index
                    let index = Box::new(AstArrayIndex::new(left, self.parse_expression()?));

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

    pub(crate) fn parse_multiplicative(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_postfix()?;

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

            let right = self.parse_postfix()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_additive(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_multiplicative()?;

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

            let right = self.parse_multiplicative()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_bitshift(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_additive()?;

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

            let right = self.parse_additive()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_compare(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_bitshift()?;

        let mut token;
        loop {
            token = self.next();
            let op = match token {
                Token::OperatorCmp('>', '>') => AstBinopOp::GT,
                Token::OperatorCmp('<', '<') => AstBinopOp::LT,
                Token::OperatorCmp('>', '=') => AstBinopOp::GE,
                Token::OperatorCmp('<', '=') => AstBinopOp::LE,
                _ => {
                    self.back();
                    break;
                }
            };

            let right = self.parse_bitshift()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_equality(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_compare()?;

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

            let right = self.parse_compare()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_bitand(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_equality()?;

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

            let right = self.parse_equality()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_bitor(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        let mut left = self.parse_bitand()?;

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

            let right = self.parse_bitand()?;

            left = Box::new(AstBinop::new(left, op, right));
        }

        Ok(left)
    }

    pub(crate) fn parse_expression(
        &mut self,
    ) -> Result<Box<dyn CompileStatementExpression>, error::Error> {
        return self.parse_bitor();
    }
}
