use zenlang::platform;

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
}
