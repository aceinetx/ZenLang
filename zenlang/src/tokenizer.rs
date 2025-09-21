use alloc::string::*;
use libm::pow;

#[derive(Debug, Clone)]
pub enum Token {
    Fn,
    Return,
    Let,
    Number(f64),
    Identifier(String),
    String(String),
    Operator(char),
    Null,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Semicolon,
    Comma,
    Assign,
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

        return Token::String(string);
    }

    pub fn _peek(&mut self) -> Token {
        let pos = self.pos;
        let token = self.next();
        self.pos = pos;
        return token;
    }

    pub fn _peek_n(&mut self, n: usize) -> Token {
        let pos = self.pos;
        let mut token: Token = Token::EOF;
        for _ in 0..n {
            token = self.next();
        }
        self.pos = pos;
        return token;
    }

    pub fn next(&mut self) -> Token {
        while self.pos < self.code.len() {
            let c = self.code.chars().nth(self.pos).unwrap();
            if self.is_digit(c) {
                let token = self.number();
                return token;
            } else if self.is_letter(c) {
                let mut token = self.identifier();
                if let Token::Identifier(ref name) = token {
                    if name == "fn" {
                        token = Token::Fn;
                    } else if name == "return" {
                        token = Token::Return;
                    } else if name == "let" {
                        token = Token::Let;
                    } else if name == "Null" {
                        token = Token::Null;
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
            } else if ['+', '-', '*', '/'].contains(&c) {
                self.pos += 1;
                return Token::Operator(c);
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
            } else if c == '=' {
                self.pos += 1;
                return Token::Assign;
            }
            self.pos += 1;
        }
        return Token::EOF;
    }
}
