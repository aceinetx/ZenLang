use crate::scope::Scope;

pub struct GlobalState {
    pub global_scope: Scope,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            global_scope: Scope::default(),
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}
