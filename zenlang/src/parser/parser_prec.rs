use crate::parser::*;
use crate::tokenizer::Token;

impl<'a> Parser<'_> {
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
}
