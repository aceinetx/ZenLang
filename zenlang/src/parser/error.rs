use crate::tokenizer::Token;
use alloc::string::String;
use alloc::{format, string::ToString};

pub enum Error {
    UnexpectedGlobalScopeToken(Token),
    FunctionSyntaxName(Token),
    FunctionSyntaxArg(Token),
    FunctionSyntaxBraces(),
    StatementSyntax(Token),
    StatementSemicolon(Token),
    LetNameSyntax(Token),
    LetDotSyntax(Token),
    LetExpectedRbracket(Token),
    LetExpectedAssign(Token),
    FunccallExpectedComma(Token),
    FunctionSyntaxHashtagBracket(Token),
    AttributeExpectedIdentifier(Token),
    UnknownAttribute(String),
    VmcallExpectedNumber(Token),
    ArrayIndexRbracket(Token),
    IndexDotSyntax(Token),
    BlockLbrace(Token),
    ExprRparen(Token),
    DictStrKey(Token),
    DictEqual(Token),
    GlobalLetIdentifier(Token),
    IfLetIdent(Token),
    IfLetEq(Token),
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
            Self::StatementSemicolon(token) => {
                format!(
                    "Expected `;` where a statement terminates, but got {:?}",
                    token
                )
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
            Self::FunctionSyntaxHashtagBracket(token) => {
                format!(
                    "Expected `[` after `#` in a function definition, but got {:?}",
                    token
                )
            }
            Self::AttributeExpectedIdentifier(token) => {
                format!(
                    "Expected an identifier as an attribute name, but got {:?}",
                    token
                )
            }
            Self::UnknownAttribute(attribute) => {
                format!("Unknown attribute: {}", attribute)
            }
            Self::VmcallExpectedNumber(token) => {
                format!("Expected number in a vmcall, but got {:?}", token)
            }
            Self::ArrayIndexRbracket(token) => {
                format!("Expected `]` after array indexing, but got {:?}", token)
            }
            Self::IndexDotSyntax(token) => {
                format!(
                    "Expected a number or an identifier after `.` while indexing, but got {:?}",
                    token
                )
            }
            Self::BlockLbrace(token) => {
                format!("Expected `{{` to begin a block, but got {:?}", token)
            }
            Self::ExprRparen(token) => {
                format!("Expected `)` to end an expression, but got {:?}", token)
            }
            Self::DictStrKey(token) => {
                format!("Expected a string as a dictionary key, but got {:?}", token)
            }
            Self::DictEqual(token) => {
                format!("Expected `=` after a dictionary key, but got {:?}", token)
            }
            Self::GlobalLetIdentifier(token) => {
                format!("Expected identifier after global let, but got {:?}", token)
            }
            Self::IfLetIdent(token) => {
                format!("Expected identifier after if let, but got {:?}", token)
            }
            Self::IfLetEq(token) => {
                format!("Expected `=` after if let identifier, but got {:?}", token)
            }
        };
    }
}
