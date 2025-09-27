//! ZenLang Platform implementation for rust's standard library
use std::fs;
use zenlang::module::Module;
use zenlang::platform;
use zenlang::stdlib::compile_stdlib_module;

pub struct Platform {}

impl Platform {
    pub fn new() -> Self {
        return Self {};
    }
}

impl platform::Platform for Platform {
    fn print(&self, s: String) {
        print!("{}", s);
    }

    fn get_string(&self) -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {}
        }
        input.trim().to_string()
    }

    fn get_module(&self, name: String) -> Option<Module> {
        if name == "stdlib" {
            let module = compile_stdlib_module();
            return Some(module);
        } else {
            let filename = name + ".zenc";
            if let Some(bytes) = self.read_file_bytes(filename) {
                let mut module = Module::new();
                if module.load(bytes).is_err() {
                    return None;
                }
                return Some(module);
            }
        }
        return None;
    }

    fn read_file_bytes(&self, name: String) -> Option<Vec<u8>> {
        match fs::read(name) {
            Err(_) => {
                return None;
            }
            Ok(bytes) => {
                return Some(bytes);
            }
        }
    }

    fn write_file_bytes(&self, name: String, bytes: Vec<u8>) {
        let _ = fs::write(name, bytes);
    }
}
