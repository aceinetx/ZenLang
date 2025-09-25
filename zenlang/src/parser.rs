use crate::ast::*;
use crate::tokenizer::*;
use alloc::boxed::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

pub struct Parser<'a> {
    pub root: root::AstRoot,
    tokenizer: &'a mut Tokenizer,
    current_token: Token,
}

impl<'a> Parser<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Parser<'a> {
        return Parser {
            root: root::AstRoot::new(),
            tokenizer: tokenizer,
            current_token: Token::EOF,
        };
    }

    fn get_token_precedence(&mut self, token: &Token) -> Option<i32> {
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

    fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        self.current_token = token.clone();
        return token;
    }

    fn error(&self, text: &str) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }

    fn error_str(&self, text: String) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }

    pub fn parse_expression(
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

    pub fn parse_statement(&mut self) -> Result<Option<Box<dyn node::Compile>>, String> {
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
            Token::Let => {
                let mut node = var_assign::AstAssign::new();
                let name;

                if let Token::Identifier(ident_name) = self.next() {
                    name = ident_name;
                } else {
                    return Err(self.error("expected identifier after let"));
                }

                self.next();
                // We want to index into the dictonary/array
                if matches!(self.current_token, Token::Lbracket) {
                    let mut node = array_assign::AstArrayAssign::new();
                    node.name = name;
                    loop {
                        // Still want to index into
                        if matches!(self.current_token, Token::Lbracket) {
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

    pub fn parse_block(&mut self) -> Result<Vec<Box<dyn node::Compile>>, String> {
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

    pub fn parse_function(&mut self) -> Result<(), String> {
        let token = self.next();
        if let Token::Identifier(name) = token {
            let mut function = function::AstFunction::new();
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

    pub fn parse_if_chain(&mut self) -> Result<Box<dyn node::Compile>, String> {
        let mut chain = if_chain::AstIfChain::new();
        loop {
            let token = self.current_token.clone();
            match token {
                Token::If => {
                    if chain.head.is_some() {
                        // If we already have chain.head defined - it means we exited the chain, so break the loop
                        break;
                    }

                    let mut node = if_stmt::AstIfStmt::new();
                    match self.parse_expression(0, true) {
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
                Token::Elif => {
                    let mut node = elif_stmt::AstElifStmt::new();
                    match self.parse_expression(0, true) {
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
