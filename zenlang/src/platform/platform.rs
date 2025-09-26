use crate::module::Module;
use alloc::string::*;

/// Platform trait
///
/// This should be implemented and passed to the VM, otherwise OS dependant features won't work

pub trait Platform {
    fn print(&self, s: String);
    fn println(&self, s: String) {
        self.print(s + "\n");
    }
    fn get_string(&self) -> String;
    fn get_module(&self, _name: String) -> Option<Module> {
        return None;
    }
}
