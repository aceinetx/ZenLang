use crate::ast::block::AstBlock;
use crate::parser::*;
use crate::tokenizer::Token;

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
