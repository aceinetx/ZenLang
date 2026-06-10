use alloc::{format, string::ToString};

use crate::tokenizer::Token;

pub enum Error {
    UnexpectedGlobalScopeToken(Token),
    FunctionSyntaxName(Token),
    FunctionSyntaxArg(Token),
    FunctionSyntaxBraces(),
    StatementSyntax(Token),
    StatementSemicolon(),
}

impl ToString for Error {
    fn to_string(&self) -> alloc::string::String {
        return match self {
            Self::UnexpectedGlobalScopeToken(token) => {
                format!("Unexpected token at the global scope: {:?}", token)
            }
            Self::FunctionSyntaxName(token) => {
                format!("Expected identifier, got {:?} as function name", token)
            }
            Self::FunctionSyntaxBraces() => {
                format!("Expected braces after function arguments")
            }
            Self::FunctionSyntaxArg(token) => {
                format!(
                    "Expected identifier as a function argument, but found {:?}",
                    token
                )
            }
            Self::StatementSyntax(token) => {
                format!("Unexpected {:?} where a statement should be", token)
            }
            Self::StatementSemicolon() => {
                format!("Expected `;` where a statement terminates")
            }
        };
    }
}
