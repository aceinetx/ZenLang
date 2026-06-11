use crate::ast::function::AstFunction;
use crate::tokenizer::Token;
use crate::{FunctionAttribute, parser::*};

impl Parser<'_> {
    pub(crate) fn parse_function(&mut self) -> Result<AstFunction, error::Error> {
        let mut func = AstFunction::new();

        match self.next() {
            Token::Hashtag => {
                let lb = self.next();
                if !matches!(lb, Token::Lbracket) {
                    return Err(error::Error::FunctionSyntaxHashtagBracket(lb));
                }

                let mut token = self.next();
                loop {
                    let name = match token {
                        Token::Identifier(ident) => ident,
                        Token::Rbracket => break,
                        _ => return Err(error::Error::AttributeExpectedIdentifier(token)),
                    };

                    let attr = FunctionAttribute::map(&name);

                    func.attrs.push(match attr {
                        Some(attr) => attr,
                        None => return Err(error::Error::UnknownAttribute(name)),
                    });

                    token = self.next();
                    match token {
                        Token::Comma => token = self.next(),
                        Token::Rbracket => break,
                        _ => (),
                    }
                }
            }
            _ => self.back(),
        }
        let name = self.next();
        let name = match name {
            Token::Identifier(ident) => ident,
            _ => return Err(error::Error::FunctionSyntaxName(name)),
        };

        func.name = name;

        // Parse arguments
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

        // Parse body
        loop {
            if matches!(self.next(), Token::Rbrace) {
                break;
            }

            self.back();

            func.body.push(self.parse_statement()?);
        }

        Ok(func)
    }
}
