use alloc::{format, string::ToString};

use crate::tokenizer::Token;

pub enum Error {
    UnexpectedGlobalScopeToken(Token),
    FunctionSyntaxName(Token),
    FunctionSyntaxArg(Token),
    FunctionSyntaxBraces(),
    StatementSyntax(Token),
    StatementSemicolon(),
    LetNameSyntax(Token),
    LetDotSyntax(Token),
    LetExpectedRbracket(Token),
    LetExpectedAssign(Token),
    FunccallExpectedComma(Token),
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
                    "Expected identifier as a function argument, but got {:?}",
                    token
                )
            }
            Self::StatementSyntax(token) => {
                format!("Unexpected {:?} where a statement should be", token)
            }
            Self::StatementSemicolon() => {
                format!("Expected `;` where a statement terminates")
            }
            Self::LetNameSyntax(token) => {
                format!("Expected identifier as a name for let, but got {:?}", token)
            }
            Self::LetDotSyntax(token) => {
                format!(
                    "Expected a number or an identifier after `.` in let, but got {:?}",
                    token
                )
            }
            Self::LetExpectedRbracket(token) => {
                format!(
                    "Expected `]` after index expression in let, but got {:?}",
                    token
                )
            }
            Self::LetExpectedAssign(token) => {
                format!("Expected `=` in let, but got {:?}", token)
            }
            Self::FunccallExpectedComma(token) => {
                format!(
                    "Expected `,` after an argument in a function call, but got {:?}",
                    token
                )
            }
        };
    }
}
