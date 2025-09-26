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
        }
        return None;
    }
}
