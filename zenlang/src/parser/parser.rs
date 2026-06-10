use alloc::boxed::Box;

use crate::ast::*;
use crate::parser::error;
use crate::tokenizer::*;

pub struct Parser<'a> {
    pub root: root::AstRoot,
    pub(crate) tokenizer: &'a mut Tokenizer,
}

impl<'a> Parser<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Parser<'a> {
        return Parser {
            root: root::AstRoot::new(),
            tokenizer: tokenizer,
        };
    }

    /// Steps a token once
    ///
    /// Saves the next token to the self.current_token
    pub(crate) fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        return token;
    }

    /// Steps back a token
    pub(crate) fn back(&mut self) {
        self.tokenizer.back();
    }

    /// Parse everything to self.root
    pub fn parse(&mut self) -> Result<(), error::Error> {
        self.root = root::AstRoot::new();

        let mut token = self.next();
        while !matches!(token, Token::EOF) {
            match token {
                Token::Fn => {
                    let func = match self.parse_function() {
                        Ok(func) => func,
                        Err(e) => return Err(e),
                    };
                    self.root.children.push(Box::new(func));
                }
                _ => {}
            }
            token = self.next();
        }
        Ok(())
    }
}
