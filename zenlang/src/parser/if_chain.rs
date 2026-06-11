use crate::ast::array::AstArray;
use crate::ast::array_index::AstArrayIndex;
use crate::ast::binop::{AstBinop, AstBinopOp};
use crate::ast::boolean::AstBoolean;
use crate::ast::elif_stmt::AstElifStmt;
use crate::ast::else_stmt::AstElseStmt;
use crate::ast::func_call::AstFuncCall;
use crate::ast::if_chain::AstIfChain;
use crate::ast::if_stmt::AstIfStmt;
use crate::ast::node::{Compile, CompileStatementExpression};
use crate::ast::null::AstNull;
use crate::ast::number::AstNumber;
use crate::ast::string::AstString;
use crate::ast::var_ref::AstVarRef;
use crate::parser::unwrap_or_ret_error;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::Box;

impl Parser<'_> {
    pub(crate) fn parse_if_chain(&mut self) -> Result<AstIfChain, error::Error> {
        let mut chain = AstIfChain::new();

        let expr = self.parse_expression()?;
        let mut if_stmt = AstIfStmt::new();
        if_stmt.value = Some(expr);
        if_stmt.body = self.parse_block()?;
        chain.head = Some(if_stmt);

        loop {
            match self.next() {
                Token::Elif => {
                    let expr = self.parse_expression()?;
                    let mut stmt = AstElifStmt::new();
                    stmt.value = Some(expr);
                    stmt.body = self.parse_block()?;
                    chain.elifs.push(stmt);
                }
                Token::Else => {
                    let mut stmt = AstElseStmt::new();
                    stmt.body = self.parse_block()?;
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
