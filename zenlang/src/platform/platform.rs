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
}
