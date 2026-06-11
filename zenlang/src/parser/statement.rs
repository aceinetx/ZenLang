use crate::ast::array_assign::AstArrayAssign;
use crate::ast::node::Compile;
use crate::ast::number::AstNumber;
use crate::ast::ret::AstReturn;
use crate::ast::var_assign::AstAssign;
use crate::ast::var_ref::AstVarRef;
use crate::ast::vmcall::AstVmcall;
use crate::parser::unwrap_or_ret_error;
use crate::parser::*;
use crate::tokenizer::Token::{self, Vmcall};
use alloc::boxed::Box;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> Result<Box<dyn Compile>, error::Error> {
        let token = self.next();
        let statement: Box<dyn Compile> = match token {
            Token::Return => {
                let node = Box::new(AstReturn::new(unwrap_or_ret_error!(
                    self.parse_expression()
                )));

                node
            }
            Token::Let => {
                let name = self.next();
                let name = match name {
                    Token::Identifier(ident) => ident,
                    _ => return Err(error::Error::LetNameSyntax(name)),
                };

                let mut assign = Box::new(AstAssign::new());
                assign.name = name.clone();
                let mut array_assign = Box::new(AstArrayAssign::new());
                array_assign.name = name;
                let mut is_array = false;

                let mut token = self.next();
                loop {
                    match token {
                        Token::Lbracket => {
                            is_array = true;
                            let expr = unwrap_or_ret_error!(self.parse_expression());

                            array_assign.indexes.push(expr);

                            let rb = self.next();
                            if !matches!(rb, Token::Rbracket) {
                                return Err(error::Error::LetExpectedRbracket(token));
                            }

                            token = self.next();
                        }
                        Token::Dot => {
                            is_array = true;

                            let index = self.next();
                            let index: Box<dyn Compile> = match index {
                                Token::Number(number) => Box::new(AstNumber::new(number)),
                                Token::Identifier(ident) => Box::new(AstVarRef::new(ident)),
                                _ => return Err(error::Error::LetDotSyntax(index)),
                            };

                            array_assign.indexes.push(index);

                            token = self.next();
                        }
                        _ => break,
                    }
                }

                if !matches!(token, Token::Assign) {
                    return Err(error::Error::LetExpectedAssign(token));
                }

                let expr = unwrap_or_ret_error!(self.parse_expression());

                if is_array {
                    array_assign.expr = Some(expr);
                    array_assign
                } else {
                    assign.expr = Some(expr);
                    assign
                }
            }
            Token::Vmcall => {
                let id = self.next();
                let id = match id {
                    Token::Number(number) => number as u8,
                    _ => return Err(error::Error::VmcallExpectedNumber(id)),
                };

                let vmcall = Box::new(AstVmcall::new(id));

                vmcall
            }
            _ => {
                self.back();
                let mut expr = unwrap_or_ret_error!(self.parse_postfix());
                expr.disable_push();

                expr
            }
        };

        if !matches!(self.next(), Token::Semicolon) {
            return Err(error::Error::StatementSemicolon());
        }

        Ok(statement)
    }
}
