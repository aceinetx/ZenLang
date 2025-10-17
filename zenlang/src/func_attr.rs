use alloc::string::*;

#[derive(PartialEq)]
pub enum FunctionAttribute {
    Naked,
    Ctor,
}

impl FunctionAttribute {
    pub fn map(name: String) -> Option<FunctionAttribute> {
        if name == "naked" {
            return Some(FunctionAttribute::Naked);
        } else if name == "ctor" {
            return Some(FunctionAttribute::Ctor);
        }
        return None;
    }
}
