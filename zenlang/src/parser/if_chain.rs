use crate::ast::elif_stmt::AstElifStmt;
use crate::ast::else_stmt::AstElseStmt;
use crate::ast::if_chain::AstIfChain;
use crate::ast::if_stmt::AstIfStmt;
use crate::parser::*;
use crate::tokenizer::Token;

impl Parser<'_> {
    pub(crate) fn parse_if_chain(&mut self) -> Result<AstIfChain, error::Error> {
        let mut chain = AstIfChain::new();

        let expr = self.parse_expression()?;
        let mut if_stmt = AstIfStmt::new();
        if_stmt.value = Some(expr);
        if_stmt.block = self.parse_block()?;
        chain.head = Some(if_stmt);

        loop {
            match self.next() {
                Token::Elif => {
                    let expr = self.parse_expression()?;
                    let mut stmt = AstElifStmt::new();
                    stmt.value = Some(expr);
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
