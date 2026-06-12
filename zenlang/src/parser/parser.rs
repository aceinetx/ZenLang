use alloc::boxed::Box;

use crate::ast::global_var::AstGlobalVar;
use crate::ast::node::Compile;
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
    pub(crate) fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        return token;
    }

    /// Steps back a token
    pub(crate) fn back(&mut self) {
        self.tokenizer.back();
    }

    /// Steps a token once, then steps back
    pub(crate) fn peek(&mut self) -> Token {
        let token = self.next();
        self.back();
        return token;
    }

    /// Parse everything to self.root
    pub fn parse(&mut self) -> Result<(), error::Error> {
        self.root = root::AstRoot::new();

        let mut token = self.next();
        while !matches!(token, Token::EOF) {
            let node: Box<dyn Compile> = match token {
                Token::Let => {
                    let name = self.next();
                    let name = match name {
                        Token::Identifier(name) => name,
                        _ => return Err(error::Error::GlobalLetIdentifier(name)),
                    };

                    let semi = self.next();
                    if !matches!(semi, Token::Semicolon) {
                        return Err(error::Error::StatementSemicolon(semi));
                    }

                    let node = Box::new(AstGlobalVar::new(name));
                    node
                }
                Token::Fn => Box::new(self.parse_function()?),
                _ => return Err(error::Error::UnexpectedGlobalScopeToken(token)),
            };
            self.root.children.push(node);
            token = self.next();
        }
        Ok(())
    }
}
