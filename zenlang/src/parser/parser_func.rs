use crate::ast::*;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::*;
use alloc::string::*;

impl<'a> Parser<'_> {
    /// Parses function
    pub(crate) fn parse_function(&mut self) -> Result<(), String> {
        let mut function = function::AstFunction::new();

        let mut token = self.next();
        if matches!(token, Token::Hashtag) {
            if !matches!(self.next(), Token::Lbracket) {
                return Err(self.error("expected `[` after `#`"));
            }
            token = self.next();

            loop {
                if let Token::Identifier(name) = token {
                    let attr = crate::FunctionAttribute::map(name);
                    if let Some(attr) = attr {
                        function.attrs.push(attr);
                    } else {
                        return Err(self.error("no such attribute"));
                    }
                } else {
                    return Err(self.error("attribute name should be an identifier"));
                }

                token = self.next();
                if matches!(token, Token::Rbracket) {
                    token = self.next();
                    break;
                }

                if !matches!(token, Token::Comma) {
                    return Err(self.error("expected `,` attribute name"));
                }

                token = self.next();
            }
        }

        if let Token::Identifier(name) = token {
            function.name = name;

            // Parse function arguments
            loop {
                let token = self.next();
                if matches!(token, Token::Lbrace) {
                    // Got a `{` - break out
                    break;
                }

                if let Token::Identifier(name) = token {
                    function.args.push(name);
                } else {
                    return Err(self.error("expected identifier in `fn <args> (HERE)`"));
                }
            }

            match self.parse_block() {
                Err(e) => {
                    return Err(e);
                }
                Ok(nodes) => {
                    function.children = nodes;
                }
            }

            self.root.children.push(Box::new(function));
        } else {
            return Err(self.error("expected identifier after fn"));
        }
        Ok(())
    }
}
