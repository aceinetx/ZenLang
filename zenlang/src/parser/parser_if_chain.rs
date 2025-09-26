use crate::ast::*;
use crate::parser::parser::Parser;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::string::*;

impl<'a> Parser<'_> {
    /// Parses an if chain
    pub(crate) fn parse_if_chain(&mut self) -> Result<Box<dyn node::Compile>, String> {
        let mut chain = if_chain::AstIfChain::new();
        loop {
            let token = self.current_token.clone();
            match token {
                Token::If => {
                    if chain.head.is_some() {
                        // If we already have chain.head defined - it means we exited the chain, so break the loop
                        break;
                    }

                    match self.next() {
                        Token::Let => {
                            let mut node = if_stmt::AstIfStmt::new();
                            node.if_let = true;

                            if let Token::Identifier(name) = self.next() {
                                node.if_let_name = name;
                            } else {
                                return Err(self.error("expected identifier after if-let. Note: if-let is not like regular let, it doesn't support indexing, etc..."));
                            }

                            if !matches!(self.next(), Token::Assign) {
                                return Err(
                                    self.error("expected semicolon after if-let <identifier>")
                                );
                            }

                            match self.parse_expression(0, true) {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(expr) => {
                                    node.if_let_expr = Some(expr);
                                }
                            }

                            match self.parse_block() {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(block) => {
                                    node.body = block;
                                }
                            }
                            self.next();
                            chain.head = Some(node);
                        }
                        _ => {
                            let mut node = if_stmt::AstIfStmt::new();
                            match self.parse_expression(0, false) {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(expr) => {
                                    node.value = Some(expr);
                                }
                            }
                            match self.parse_block() {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(block) => {
                                    node.body = block;
                                }
                            }
                            self.next();
                            chain.head = Some(node);
                        }
                    }
                }
                Token::Elif => match self.next() {
                    Token::Let => {
                        let mut node = elif_stmt::AstElifStmt::new();
                        node.elif_let = true;

                        if let Token::Identifier(name) = self.next() {
                            node.elif_let_name = name;
                        } else {
                            return Err(self.error("expected identifier after if-let. Note: if-let is not like regular let, it doesn't support indexing, etc..."));
                        }

                        if !matches!(self.next(), Token::Assign) {
                            return Err(self.error("expected semicolon after if-let <identifier>"));
                        }

                        match self.parse_expression(0, true) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(expr) => {
                                node.elif_let_expr = Some(expr);
                            }
                        }

                        match self.parse_block() {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(block) => {
                                node.body = block;
                            }
                        }

                        self.next();
                        chain.elifs.push(node);
                    }
                    _ => {
                        let mut node = elif_stmt::AstElifStmt::new();
                        match self.parse_expression(0, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(expr) => {
                                node.value = Some(expr);
                            }
                        }
                        match self.parse_block() {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(block) => {
                                node.body = block;
                            }
                        }
                        self.next();
                        chain.elifs.push(node);
                    }
                },
                Token::Else => {
                    let mut node = else_stmt::AstElseStmt::new();
                    self.next();
                    match self.parse_block() {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(block) => {
                            node.body = block;
                        }
                    }
                    self.next();
                    chain.else_node = Some(node);
                    break; // Guaranteed to be the end of the chain - break out of the loop
                }
                _ => {
                    break;
                }
            }
        }
        return Ok(Box::new(chain));
    }
}
