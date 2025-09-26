use crate::ast::*;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

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

    /// Get the token precedence from a given token
    pub(crate) fn get_token_precedence(&mut self, token: &Token) -> Option<i32> {
        match *token {
            Token::Operator(op) => {
                if op == '+' {
                    return Some(6);
                } else if op == '-' {
                    return Some(6);
                } else if op == '*' {
                    return Some(7);
                } else if op == '/' {
                    return Some(7);
                }
                return None;
            }
            Token::OperatorCmp(first, _) => {
                if first == '=' {
                    return Some(4);
                } else if first == '!' {
                    return Some(4);
                } else if first == '>' {
                    return Some(5);
                } else if first == '<' {
                    return Some(5);
                }
                return None;
            }
            Token::BitOperator(first, _) => {
                if first == '|' {
                    return Some(1);
                } else if first == '&' {
                    return Some(2);
                } else if first == '<' {
                    return Some(3);
                } else if first == '>' {
                    return Some(3);
                }
                return None;
            }
            _ => {
                return None;
            }
        }
    }

    /// Steps a token once
    ///
    /// Saves the next token to the self.current_token
    pub(crate) fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        self.current_token = token.clone();
        return token;
    }

    /// Formats an error message like this:
    ///
    /// 1: error text
    pub(crate) fn error(&self, text: &str) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }

    /// Formats an error message like this:
    ///
    /// 1: error text
    pub(crate) fn error_str(&self, text: String) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }

    /// Parses a code block
    pub(crate) fn parse_block(&mut self) -> Result<Vec<Box<dyn node::Compile>>, String> {
        let mut vec: Vec<Box<dyn node::Compile>> = Vec::new();

        self.next();
        loop {
            if matches!(self.current_token, Token::Rbrace) {
                // End of the block - break out
                break;
            }

            match self.parse_statement() {
                Err(e) => {
                    return Err(e);
                }
                Ok(node_option) => {
                    // parse_statement can return None in case if the statement was one semicolon (valid syntax)
                    if let Some(node) = node_option {
                        vec.push(node);
                    }
                }
            }
        }

        Ok(vec)
    }

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
                _ => {
                    return Err(self.error("unexpected token, expected one of these: Fn"));
                }
            }
            token = self.next();
        }
        Ok(())
    }
}
