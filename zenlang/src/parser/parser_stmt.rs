use crate::ast::*;
use crate::parser::parser::Parser;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;

impl<'a> Parser<'_> {
    /// Parses statements
    pub(crate) fn parse_statement(&mut self) -> Result<Option<Box<dyn node::Compile>>, String> {
        let token = self.current_token.clone();

        match token {
            Token::Return => match self.parse_expression(0, true) {
                Err(e) => {
                    return Err(e);
                }
                Ok(node) => {
                    // We expect a semicolon after the expression
                    if !matches!(self.current_token, Token::Semicolon) {
                        return Err(self.error_str(format!(
                            "expected semicolon after return, found {:?}",
                            self.current_token
                        )));
                    }

                    let mut ret = ret::AstReturn::new();
                    ret.value = Some(node);
                    self.next();
                    return Ok(Some(Box::new(ret)));
                }
            },
            Token::Vmcall => {
                let mut node = vmcall::AstVmcall::new();

                if let Token::Number(id_f64) = self.next() {
                    node.id = id_f64 as u8;
                } else {
                    return Err(self.error("expected number after vmcall"));
                }

                if !matches!(self.next(), Token::Semicolon) {
                    return Err(self.error("expected semicolon after vmcall"));
                }
                self.next();

                return Ok(Some(Box::new(node)));
            }
            Token::Dynmod => match self.parse_expression(0, true) {
                Err(e) => {
                    return Err(e);
                }
                Ok(node) => {
                    // We expect a semicolon after the expression
                    if !matches!(self.current_token, Token::Semicolon) {
                        return Err(self.error_str(format!(
                            "expected semicolon after dynmod, found {:?}",
                            self.current_token
                        )));
                    }

                    let mut dynmod = dynmod_stmt::AstDynmod::new();
                    dynmod.name = Some(node);
                    self.next();
                    return Ok(Some(Box::new(dynmod)));
                }
            },
            Token::Let => {
                let mut node = var_assign::AstAssign::new();
                let name;

                if let Token::Identifier(ident_name) = self.next() {
                    name = ident_name;
                } else {
                    return Err(self.error("expected identifier after let"));
                }

                self.next();
                if matches!(self.current_token, Token::Lbracket)
                    || matches!(self.current_token, Token::Dot)
                {
                    // We want to index into the dictonary/array
                    let mut node = array_assign::AstArrayAssign::new();
                    node.name = name;
                    loop {
                        // Still want to index into
                        if matches!(self.current_token, Token::Lbracket) {
                            // Bracket indexing
                            match self.parse_expression(0, true) {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(index) => {
                                    node.indexes.push(index);
                                }
                            }

                            if !matches!(self.current_token, Token::Rbracket) {
                                return Err(self.error("expected `]`"));
                            }
                        } else if matches!(self.current_token, Token::Dot) {
                            // Dotted indexing
                            self.next();
                            match self.current_token.clone() {
                                Token::Number(num) => {
                                    let mut index = number::AstNumber::new();
                                    index.number = num;
                                    node.indexes.push(Box::new(index));
                                }
                                Token::Identifier(key) => {
                                    let mut index = string::AstString::new();
                                    index.string = key;
                                    node.indexes.push(Box::new(index));
                                }
                                _ => {
                                    return Err(self.error("Expected either a constant string or a identifier in a dot index. Tip: use [expr] for indexing with runtime values"));
                                }
                            }
                        } else if matches!(self.current_token, Token::Assign) {
                            // Oh! We got a assign operator - parse the expression
                            match self.parse_expression(0, true) {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(expr) => {
                                    // We expect a semicolon after the expression
                                    if !matches!(self.current_token, Token::Semicolon) {
                                        return Err(self.error("expected semicolon after let"));
                                    }

                                    node.expr = Some(expr);
                                    self.next();
                                    return Ok(Some(Box::new(node)));
                                }
                            }
                        }

                        self.next();
                    }
                } else if !matches!(self.current_token, Token::Assign) {
                    // If we don't want to index, we expect a =
                    return Err(self.error("expected `=` after let <ident>"));
                }

                // Parse the assign expression
                match self.parse_expression(0, true) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(expr) => {
                        // We expect a semicolon after the expression
                        if !matches!(self.current_token, Token::Semicolon) {
                            return Err(self.error("expected semicolon after let"));
                        }

                        node.expr = Some(expr);
                        node.name = name;
                        self.next();
                        return Ok(Some(Box::new(node)));
                    }
                }
            }
            Token::Semicolon => {
                // Making a whole ast struct just for semicolons is unnecessary, just return None
                self.next();
                return Ok(None);
            }
            Token::If => {
                //
                match self.parse_if_chain() {
                    Ok(node) => {
                        return Ok(Some(node));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Break => {
                let node = break_stmt::AstBreak::new();
                self.next();
                if !matches!(self.current_token, Token::Semicolon) {
                    return Err(self.error("expected semicolon after break"));
                }
                return Ok(Some(Box::new(node)));
            }
            Token::Continue => {
                let node = continue_stmt::AstContinue::new();
                self.next();
                if !matches!(self.current_token, Token::Semicolon) {
                    return Err(self.error("expected semicolon after contin7e"));
                }
                return Ok(Some(Box::new(node)));
            }
            Token::While => {
                let mut node = while_stmt::AstWhileStmt::new();
                match self.parse_expression(0, true) {
                    Err(e) => return Err(e),
                    Ok(expr) => {
                        node.value = Some(expr);
                    }
                }
                match self.parse_block() {
                    Err(e) => return Err(e),
                    Ok(body) => {
                        node.body = body;
                    }
                }
                self.next();
                return Ok(Some(Box::new(node)));
            }
            _ => match self.parse_expression(0, false) {
                Err(e) => {
                    return Err(e);
                }
                Ok(mut expr) => {
                    expr.disable_push();

                    if !matches!(self.current_token, Token::Semicolon) {
                        return Err(self.error("expected semicolon after expression"));
                    }

                    return Ok(Some(expr));
                }
            },
        }
    }
}
