//! Tokenizer
//!
//! Tokenizes ZenLang code into tokens
//!
//! ### Example
//! `fn main {}` -> `Fn, Identifier(main), Lbrace, Rbrace`
use alloc::string::*;
use libm::pow;
use unescape;

#[derive(Debug, Clone)]
pub enum Token {
    Fn,
    Return,
    Let,
    Number(f64),
    Identifier(String),
    String(String),
    Operator(char),
    OperatorCmp(char, char),
    BitOperator(char, char),
    Null,
    True,
    False,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Semicolon,
    Comma,
    Dot,
    Assign,
    Hashtag,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    Vmcall,
    Mod,
    Dynmod,
    EOF,
}

#[derive(Debug)]
pub struct Tokenizer {
    code: String,
    pos: usize,
}

impl Tokenizer {
    pub fn new(code: String) -> Tokenizer {
        return Tokenizer { code: code, pos: 0 };
    }

    fn is_digit(&self, ch: char) -> bool {
        return ch >= '0' && ch <= '9';
    }

    fn is_letter(&self, ch: char) -> bool {
        return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
    }

    fn is_identifier_letter(&self, ch: char) -> bool {
        return self.is_letter(ch) || ch == '_';
    }

    fn number(&mut self) -> Token {
        let mut num = 0.0;

        let mut decimal_part = false;
        let mut decmial_nums: u64 = 1;

        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos).unwrap();
            if c == '.' {
                decimal_part = true;
                self.pos += 1;
                continue;
            } else if !self.is_digit(c) {
                break;
            }

            if !decimal_part {
                // int part
                num *= 10.0;
                num += (c as u8 - '0' as u8) as f64;
            } else {
                // decimal part
                let digit = (c as u8 - '0' as u8) as f64;
                let part = digit / (pow(10.0, decmial_nums as f64));
                num += part;
                decmial_nums += 1;
            }

            self.pos += 1;
        }

        return Token::Number(num);
    }

    fn identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos).unwrap();
            if !self.is_identifier_letter(c) {
                break;
            }

            identifier.push(c);

            self.pos += 1;
        }

        return Token::Identifier(identifier);
    }

    fn string(&mut self) -> Token {
        let mut string = String::new();
        self.pos += 1;
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos).unwrap();
            if c == '"' {
                self.pos += 1;
                break;
            }

            string.push(c);

            self.pos += 1;
        }

        if let Some(s) = unescape::unescape(string.as_str()) {
            string = s;
        }
        return Token::String(string);
    }

    pub fn get_line(&self) -> u64 {
        let mut lines = 1;
        for i in 0..self.code.len() {
            if i >= self.pos {
                break;
            }

            if self.code.chars().nth(i).unwrap() == '\n' {
                lines += 1;
            }
        }
        return lines;
    }

    pub fn next(&mut self) -> Token {
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos).unwrap();
            if self.is_digit(c) {
                let token = self.number();
                return token;
            } else if self.is_identifier_letter(c) {
                let mut token = self.identifier();
                if let Token::Identifier(ref name) = token {
                    if name == "fn" {
                        token = Token::Fn;
                    } else if name == "return" {
                        token = Token::Return;
                    } else if name == "let" {
                        token = Token::Let;
                    } else if name == "null" {
                        token = Token::Null;
                    } else if name == "true" {
                        token = Token::True;
                    } else if name == "false" {
                        token = Token::False;
                    } else if name == "if" {
                        token = Token::If;
                    } else if name == "elif" {
                        token = Token::Elif;
                    } else if name == "else" {
                        token = Token::Else;
                    } else if name == "while" {
                        token = Token::While;
                    } else if name == "break" {
                        token = Token::Break;
                    } else if name == "continue" {
                        token = Token::Continue;
                    } else if name == "vmcall" {
                        token = Token::Vmcall;
                    } else if name == "mod" {
                        token = Token::Mod;
                    } else if name == "dynmod" {
                        token = Token::Dynmod;
                    }
                }
                return token;
            } else if c == '"' {
                let token = self.string();
                return token;
            } else if c == '{' {
                self.pos += 1;
                return Token::Lbrace;
            } else if c == '}' {
                self.pos += 1;
                return Token::Rbrace;
            } else if c == ';' {
                self.pos += 1;
                return Token::Semicolon;
            } else if c == ',' {
                self.pos += 1;
                return Token::Comma;
            } else if c == '.' {
                self.pos += 1;
                return Token::Dot;
            } else if ['+', '-', '*', '/'].contains(&c) {
                self.pos += 1;
                if self.pos < self.code.len() {
                    let c = self.code.chars().nth(self.pos).unwrap();
                    if c == '/' {
                        while self.pos < self.code.len() {
                            if self.code.chars().nth(self.pos).unwrap() == '\n' {
                                self.pos += 1;
                                break;
                            }
                            self.pos += 1;
                        }
                        continue;
                    }
                }
                return Token::Operator(c);
            } else if c == '|' {
                self.pos += 1;
                return Token::BitOperator('|', '|');
            } else if c == '&' {
                self.pos += 1;
                return Token::BitOperator('&', '&');
            } else if c == '(' {
                self.pos += 1;
                return Token::Lparen;
            } else if c == ')' {
                self.pos += 1;
                return Token::Rparen;
            } else if c == '[' {
                self.pos += 1;
                return Token::Lbracket;
            } else if c == ']' {
                self.pos += 1;
                return Token::Rbracket;
            } else if c == '#' {
                self.pos += 1;
                return Token::Hashtag;
            } else if c == '=' {
                self.pos += 1;
                if self.pos < self.code.len() {
                    let c = self.code.chars().nth(self.pos).unwrap();
                    if c == '=' {
                        self.pos += 1;
                        return Token::OperatorCmp('=', '=');
                    }
                }
                return Token::Assign;
            } else if c == '>' {
                self.pos += 1;
                if self.pos < self.code.len() {
                    let c = self.code.chars().nth(self.pos).unwrap();
                    if c == '=' {
                        self.pos += 1;
                        return Token::OperatorCmp('>', '=');
                    } else if c == '>' {
                        self.pos += 1;
                        return Token::BitOperator('>', '>');
                    }
                }
                return Token::OperatorCmp('>', '>');
            } else if c == '<' {
                self.pos += 1;
                if self.pos < self.code.len() {
                    let c = self.code.chars().nth(self.pos).unwrap();
                    if c == '=' {
                        self.pos += 1;
                        return Token::OperatorCmp('<', '=');
                    } else if c == '<' {
                        self.pos += 1;
                        return Token::BitOperator('<', '<');
                    }
                }
                return Token::OperatorCmp('<', '<');
            } else if c == '!' {
                self.pos += 1;
                if self.pos < self.code.len() {
                    let c = self.code.chars().nth(self.pos).unwrap();
                    if c == '=' {
                        self.pos += 1;
                        return Token::OperatorCmp('!', '=');
                    }
                }
                return Token::Operator('!');
            }

            self.pos += 1;
        }
        return Token::EOF;
    }
}
