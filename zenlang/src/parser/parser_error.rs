use crate::parser::*;
use alloc::format;
use alloc::string::*;

impl<'a> Parser<'_> {
    /// Formats an error message like this:
    ///
    /// 1: error text
    pub(crate) fn error(&self, text: &str) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }

    /// Formats an error message like this:
    ///
    /// 1: error text
    pub(crate) fn error_str(&self, text: String) -> String {
        return format!("{}: {}", self.tokenizer.get_line(), text);
    }
}
