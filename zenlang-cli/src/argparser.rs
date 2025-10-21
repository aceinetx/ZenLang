pub struct ArgParser {
    pub filename: String,
    pub compile: bool,
}

impl ArgParser {
    pub fn new() -> Self {
        return Self {
            filename: String::new(),
            compile: false,
        };
    }

    pub fn parse(&mut self, args: &Vec<String>) {
        for arg in args {
            if arg == "-compile" {
                self.compile = true;
            } else {
                self.filename = arg.to_string();
            }
        }
    }
}
