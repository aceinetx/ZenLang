use crate::ast::*;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;

pub struct Parser<'a> {
    pub root: root::AstRoot,
    pub(crate) tokenizer: &'a mut Tokenizer,
    pub(crate) current_token: Token,
}

impl<'a> Parser<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Parser<'a> {
        return Parser {
            root: root::AstRoot::new(),
            tokenizer: tokenizer,
            current_token: Token::EOF,
        };
    }

    /// Steps a token once
    ///
    /// Saves the next token to the self.current_token
    pub(crate) fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        self.current_token = token.clone();
        return token;
    }

    /// Parse everything to self.root
    pub fn parse(&mut self) -> Result<(), String> {
        self.root = root::AstRoot::new();

        let mut token = self.next();
        while !matches!(token, Token::EOF) {
            match token {
                Token::Fn => {
                    if let Err(e) = self.parse_function() {
                        return Err(e);
                    }
                }
                Token::Let => {
                    let mut node = global_var::AstGlobalVar::new();
                    if let Token::Identifier(name) = self.next() {
                        node.name = name;
                    } else {
                        return Err(self.error("expected identifier after global let"));
                    }

                    if !matches!(self.next(), Token::Semicolon) {
                        return Err(self.error("expected semicolon after global let <name>"));
                    }

                    self.root.children.push(Box::new(node));
                }
                Token::Mod => {
                    let mut node = mod_stmt::AstMod::new();
                    if let Token::Identifier(name) = self.next() {
                        node.name = name;
                    } else {
                        return Err(self.error("expected identifier after mod"));
                    }

                    if !matches!(self.next(), Token::Semicolon) {
                        return Err(self.error("expected semicolon after mod <name>"));
                    }

                    self.root.children.push(Box::new(node));
                }
                _ => {
                    return Err(self.error_str(format!(
                        "unexpected token, expected one of these: Fn | Mod, found, {:?}",
                        token
                    )));
                }
            }
            token = self.next();
        }
        Ok(())
    }
}
