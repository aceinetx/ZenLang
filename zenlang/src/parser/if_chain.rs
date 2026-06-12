use crate::ast::elif_stmt::AstElifStmt;
use crate::ast::else_stmt::AstElseStmt;
use crate::ast::if_chain::AstIfChain;
use crate::ast::if_stmt::AstIfStmt;
use crate::parser::*;
use crate::tokenizer::Token;

impl Parser<'_> {
    pub(crate) fn parse_if_chain(&mut self) -> Result<AstIfChain, error::Error> {
        let mut chain = AstIfChain::new();
        let mut if_stmt = AstIfStmt::new();

        match self.next() {
            Token::Let => {
                let name = self.next();
                let name = match name {
                    Token::Identifier(name) => name,
                    _ => return Err(error::Error::IfLetIdent(name)),
                };

                if_stmt.if_let = true;
                if_stmt.if_let_name = name;

                let eq = self.next();
                if !matches!(eq, Token::Assign) {
                    return Err(error::Error::IfLetEq(eq));
                }

                if_stmt.if_let_expr = Some(self.parse_expression()?);
            }
            _ => {
                self.back();
                if_stmt.value = Some(self.parse_expression()?);
            }
        }

        if_stmt.block = self.parse_block()?;
        chain.head = Some(if_stmt);

        loop {
            match self.next() {
                Token::Elif => {
                    let mut stmt = AstElifStmt::new();

                    match self.next() {
                        Token::Let => {
                            let name = self.next();
                            let name = match name {
                                Token::Identifier(name) => name,
                                _ => return Err(error::Error::IfLetIdent(name)),
                            };

                            stmt.elif_let = true;
                            stmt.elif_let_name = name;

                            let eq = self.next();
                            if !matches!(eq, Token::Assign) {
                                return Err(error::Error::IfLetEq(eq));
                            }

                            stmt.elif_let_expr = Some(self.parse_expression()?);
                        }
                        _ => {
                            self.back();
                            stmt.value = Some(self.parse_expression()?);
                        }
                    }
                    stmt.block = self.parse_block()?;

                    chain.elifs.push(stmt);
                }
                Token::Else => {
                    let mut stmt = AstElseStmt::new();
                    stmt.block = self.parse_block()?;
                    chain.else_node = Some(stmt);
                    break;
                }
                _ => {
                    self.back();
                    break;
                }
            };
        }

        Ok(chain)
    }
}
