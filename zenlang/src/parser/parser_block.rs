use core::any::TypeId;

use crate::ast::*;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::*;
use alloc::string::*;
use alloc::vec::*;

impl<'a> Parser<'_> {
    /// Parses a code block
    pub(crate) fn parse_block(&mut self) -> Result<Vec<Box<dyn node::Compile>>, String> {
        let mut vec: Vec<Box<dyn node::Compile>> = Vec::new();
        let mut defers: Vec<Box<dyn node::Compile>> = Vec::new();

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
                    /*if let Some(node) = node_option {
                        if let Ok(_) = node.downcast::<defer::AstDefer>() {
                            defer_indexes.push(vec.len());
                        }
                        vec.push(node);
                    }*/
                    if let Some(node) = node_option {
                        if TypeId::of::<defer::AstDefer>() == node.type_id() {
                            defers.push(node);
                        } else {
                            vec.push(node);
                        }
                    }
                }
            }
        }

        while !defers.is_empty() {
            vec.push(defers.pop().unwrap());
        }

        Ok(vec)
    }
}
