use alloc::string::*;

pub trait Platform {
    fn print(&self, s: String);
    fn println(&self, s: String) {
        self.print(s + "\n");
    }
    fn get_string(&self) -> String;
}
