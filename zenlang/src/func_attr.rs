use alloc::string::*;

#[derive(PartialEq)]
pub enum FunctionAttribute {
    Naked,
}

impl FunctionAttribute {
    pub fn map(name: String) -> Option<FunctionAttribute> {
        if name == "naked" {
            return Some(FunctionAttribute::Naked);
        }
        return None;
    }
}
