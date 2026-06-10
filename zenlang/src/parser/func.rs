use crate::ast::function::AstFunction;
use crate::parser::*;
use crate::tokenizer::Token;
use crate::unwrap_or_ret_error;

impl Parser<'_> {
    pub(crate) fn parse_function(&mut self) -> Result<AstFunction, error::Error> {
        let name = self.next();
        let name = match name {
            Token::Identifier(ident) => ident,
            _ => return Err(error::Error::FunctionSyntaxName(name)),
        };

        let mut func = AstFunction::new();
        func.name = name;

        loop {
            let token = self.next();
            if matches!(token, Token::Lbrace) {
                break;
            }

            let arg = match token {
                Token::Identifier(ident) => ident,
                _ => return Err(error::Error::FunctionSyntaxArg(token)),
            };

            func.args.push(arg);
        }

        loop {
            if matches!(self.next(), Token::Rbrace) {
                break;
            }

            self.back();

            func.body.push(unwrap_or_ret_error!(self.parse_statement()));
        }

        Ok(func)
    }
}
