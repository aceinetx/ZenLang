use crate::ast::array::AstArray;
use crate::ast::array_index::AstArrayIndex;
use crate::ast::binop::{AstBinop, AstBinopOp};
use crate::ast::block::AstBlock;
use crate::ast::boolean::AstBoolean;
use crate::ast::func_call::AstFuncCall;
use crate::ast::if_chain::AstIfChain;
use crate::ast::node::{Compile, CompileStatementExpression};
use crate::ast::null::AstNull;
use crate::ast::number::AstNumber;
use crate::ast::string::AstString;
use crate::ast::var_ref::AstVarRef;
use crate::parser::unwrap_or_ret_error;
use crate::parser::*;
use crate::parser::*;
use crate::tokenizer::Token;
use alloc::boxed::Box;

impl Parser<'_> {
    pub(crate) fn parse_block(&mut self) -> Result<AstBlock, error::Error> {
        let mut block = AstBlock::new();

        let lb = self.next();
        if !matches!(lb, Token::Lbrace) {
            return Err(error::Error::BlockLbrace(lb));
        }

        loop {
            let token = self.next();
            if matches!(token, Token::Rbrace) {
                break;
            }
            self.back();

            block.body.push(self.parse_statement()?);
        }

        Ok(block)
    }
}
