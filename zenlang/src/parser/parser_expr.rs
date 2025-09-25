use crate::ast::*;
use crate::parser::parser::Parser;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;

impl<'a> Parser<'_> {
    /// Parses an expression
    ///
    /// * `min_prec` - Minimal precedence, 0 if just started parsing
    /// * `step_token` - Whether parse_expression should step a token once
    pub(crate) fn parse_expression(
        &mut self,
        min_prec: i32,
        step_token: bool,
    ) -> Result<Box<dyn node::Compile>, String> {
        let mut token;
        if step_token {
            token = self.next();
        } else {
            token = self.current_token.clone();
        }

        let mut left: Box<dyn node::Compile>;

        // * Parse base value
        match token {
            Token::Operator(_) => {
                let prec = self.get_token_precedence(&token).unwrap();

                self.next();
                match self.parse_expression(prec, false) {
                    Ok(node) => {
                        left = node;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Lbracket => {
                // Parse array
                let mut node = array::AstArray::new();

                loop {
                    self.next();
                    // Got a `]`, so it's the end - break out
                    if matches!(self.current_token, Token::Rbracket) {
                        break;
                    }

                    // Parse the value
                    match self.parse_expression(0, false) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(expr) => {
                            node.values.push(expr);
                        }
                    }

                    // Got a `]`, so it's the end - break out
                    if matches!(self.current_token, Token::Rbracket) {
                        break;
                    }
                    // Expect a comma after each item
                    if !matches!(self.current_token, Token::Comma) {
                        return Err(self.error("expected `,` after array item: [ <ITEMS> [HERE] ]"));
                    }
                }

                self.next();
                left = Box::new(node);
            }
            Token::Number(num) => {
                let mut node = number::AstNumber::new();
                node.number = num;
                left = Box::new(node);

                self.next();
            }
            Token::String(string) => {
                let mut node = string::AstString::new();
                node.string = string;
                left = Box::new(node);

                self.next();
            }
            Token::Lbrace => {
                // Dictionary
                let mut node = dict::AstDict::new();

                loop {
                    self.next();
                    // Got a `}`, so it's the end - break out
                    if matches!(self.current_token, Token::Rbrace) {
                        break;
                    }

                    let name;
                    if let Token::String(s) = self.current_token.clone() {
                        name = s;
                    } else {
                        // Dictionary keys are just strings in ZenLang - so expect a string
                        return Err(self.error("expected string as dict item name"));
                    }

                    self.next();
                    // We parsed the key - expect a equal sign
                    if !matches!(self.current_token, Token::Assign) {
                        return Err(self.error("expected `=` after dict item name"));
                    }
                    self.next();

                    // Finally parse the value
                    match self.parse_expression(0, false) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(expr) => {
                            node.dict.push((name, expr));
                        }
                    }

                    // Got a `}`, so it's the end - break out
                    if matches!(self.current_token, Token::Rbrace) {
                        break;
                    }
                    // Expect a semicolon after each value
                    if !matches!(self.current_token, Token::Comma) {
                        return Err(self.error("expected `,` after dict item: [ <ITEMS> [HERE] ]"));
                    }
                }

                left = Box::new(node);

                self.next();
            }
            Token::Lparen => {
                self.next();
                match self.parse_expression(0, false) {
                    Ok(node) => {
                        left = node;
                        token = self.current_token.clone();
                        if !matches!(token, Token::Rparen) {
                            return Err(self.error("expected `)`"));
                        }
                        self.next();
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Identifier(name) => {
                // function call
                let mut node = var_ref::AstVarRef::new();
                node.name = name;
                left = Box::new(node);

                self.next();
            }
            Token::Null => {
                let node = null::AstNull::new();
                left = Box::new(node);

                self.next();
            }
            Token::False => {
                let mut node = boolean::AstBoolean::new();
                node.flag = false;
                left = Box::new(node);

                self.next();
            }
            Token::True => {
                let mut node = boolean::AstBoolean::new();
                node.flag = true;
                left = Box::new(node);

                self.next();
            }
            _ => {
                return Err(self
                    .error(format!("unexpected token in parse_expression: {:?}", token).as_str()));
            }
        }

        // * Parse operators
        loop {
            token = self.current_token.clone();
            if let Token::Operator(op) = token {
                // Simple operator
                match self.get_token_precedence(&token) {
                    Some(prec) => {
                        if prec < min_prec {
                            break;
                        }

                        let next_min = prec + 1; // left assoc
                        match self.parse_expression(next_min, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(right) => {
                                let mut binop = binop::AstBinop::new();
                                binop.left = Some(left);
                                binop.right = Some(right);
                                if op == '+' {
                                    binop.op = binop::AstBinopOp::PLUS;
                                } else if op == '-' {
                                    binop.op = binop::AstBinopOp::MINUS;
                                } else if op == '*' {
                                    binop.op = binop::AstBinopOp::MUL;
                                } else if op == '/' {
                                    binop.op = binop::AstBinopOp::DIV;
                                }
                                left = Box::new(binop);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            } else if let Token::OperatorCmp(first_char, second_char) = token {
                // Compare operator
                match self.get_token_precedence(&token) {
                    Some(prec) => {
                        if prec < min_prec {
                            break;
                        }

                        let next_min = prec; // right assoc 
                        self.next();
                        match self.parse_expression(next_min, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(right) => {
                                let mut binop = binop::AstBinop::new();
                                binop.left = Some(left);
                                binop.right = Some(right);
                                if first_char == '=' && second_char == '=' {
                                    binop.op = binop::AstBinopOp::EQ;
                                } else if first_char == '!' && second_char == '=' {
                                    binop.op = binop::AstBinopOp::NEQ;
                                } else if first_char == '>' && second_char == '>' {
                                    binop.op = binop::AstBinopOp::GT;
                                } else if first_char == '<' && second_char == '<' {
                                    binop.op = binop::AstBinopOp::LT;
                                } else if first_char == '>' && second_char == '=' {
                                    binop.op = binop::AstBinopOp::GE;
                                } else if first_char == '<' && second_char == '=' {
                                    binop.op = binop::AstBinopOp::LE;
                                }
                                left = Box::new(binop);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            } else if let Token::BitOperator(first_char, _) = token {
                // Bitwise operator
                match self.get_token_precedence(&token) {
                    Some(prec) => {
                        if prec < min_prec {
                            break;
                        }

                        let next_min = prec; // right assoc 
                        self.next();
                        match self.parse_expression(next_min, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(right) => {
                                let mut binop = binop::AstBinop::new();
                                binop.left = Some(left);
                                binop.right = Some(right);
                                if first_char == '|' {
                                    binop.op = binop::AstBinopOp::BITOR;
                                } else if first_char == '&' {
                                    binop.op = binop::AstBinopOp::BITAND;
                                } else if first_char == '<' {
                                    binop.op = binop::AstBinopOp::BITSHL;
                                } else if first_char == '>' {
                                    binop.op = binop::AstBinopOp::BITSHR;
                                }
                                left = Box::new(binop);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            } else if let Token::Lbracket = token {
                // Index into a value
                match self.parse_expression(0, true) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(index) => {
                        let mut node = array_index::AstArrayIndex::new();
                        node.array = Some(left);
                        node.index = Some(index);
                        left = Box::new(node);
                    }
                }
                self.next();
            } else if let Token::Dot = token {
                // Dotted index into a value
                self.next();
                match self.current_token.clone() {
                    Token::Number(num) => {
                        let mut node = array_index::AstArrayIndex::new();
                        let mut index = number::AstNumber::new();
                        index.number = num;

                        node.array = Some(left);
                        node.index = Some(Box::new(index));
                        left = Box::new(node);
                    }
                    Token::Identifier(key) => {
                        let mut node = array_index::AstArrayIndex::new();
                        let mut index = string::AstString::new();
                        index.string = key;

                        node.array = Some(left);
                        node.index = Some(Box::new(index));
                        left = Box::new(node);
                    }
                    _ => {
                        return Err(self.error("Expected either a constant string or a identifier in a dot index. Tip: use [expr] for indexing with runtime values"));
                    }
                }
                self.next();
            } else if let Token::Lparen = token {
                let mut node = func_call::AstFuncCall::new();
                node.reference = Some(left);

                // Parse the function arguments
                loop {
                    self.next();
                    // Got `(` so it's the end - break out
                    if matches!(self.current_token, Token::Rparen) {
                        break;
                    }

                    // Parse the argument
                    match self.parse_expression(0, false) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(expr) => {
                            node.args.push(expr);
                        }
                    }

                    token = self.current_token.clone();
                    // Got `(` so it's the end - break out
                    if matches!(token, Token::Rparen) {
                        break;
                    }

                    // We didn't get a (, so it means we need to continue parsing
                    // However, we expect a semicolon after each argument, so expect it here
                    if !matches!(token, Token::Comma) {
                        return Err(self
                            .error("expected `,` after a function argument: CALL(<args> [HERE])"));
                    }
                }

                left = Box::new(node);
                self.next();
            } else {
                break;
            }
        }

        Ok(left)
    }
}
