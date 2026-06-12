use crate::ast::while_stmt::AstWhileStmt;
use crate::parser::Parser;
use crate::parser::error;

impl Parser<'_> {
    pub(crate) fn parse_while(&mut self) -> Result<AstWhileStmt, error::Error> {
        let expr = self.parse_expression()?;
        let block = self.parse_block()?;

        let mut node = AstWhileStmt::new();
        node.value = Some(expr);
        node.body = block;

        Ok(node)
    }
}
