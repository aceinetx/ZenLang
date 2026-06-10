use crate::ast;
use crate::ast::ret::AstReturn;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::Box;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> Result<Box<dyn ast::node::Compile>, error::Error> {
        let token = self.next();
        return match token {
            Token::Return => {
                let node = Box::new(AstReturn::new(unwrap_or_ret_error!(
                    self.parse_expression()
                )));

                if !matches!(self.next(), Token::Semicolon) {
                    return Err(error::Error::StatementSemicolon());
                }

                Ok(node)
            }
            _ => Err(error::Error::StatementSyntax(token)),
        };
    }
}
