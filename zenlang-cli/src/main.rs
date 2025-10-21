use std::{env, fs, io::Read, path::Path};

use crate::argparser::ArgParser;

mod argparser;
mod runner;

pub fn get_module_name_from_path(path: &String) -> String {
    let path = Path::new(&path);

    if let Some(stem) = path.file_stem() {
        let filename_without_extension = stem.to_string_lossy();
        return filename_without_extension.to_string();
    }
    return path.to_string_lossy().to_string();
}

fn main() {
    let mut args = ArgParser::new();
    args.parse(&env::args().collect::<Vec<String>>());

    if args.filename.is_empty() {
        println!("zenlang: no filename provided");
        return;
    }

    let module_name = get_module_name_from_path(&args.filename);
    match fs::File::open(&args.filename) {
        Ok(mut file) => {
            if !args.compile {
                if args.filename.ends_with(".zen") {
                    let mut text = String::new();
                    if let Err(error) = file.read_to_string(&mut text) {
                        println!("read error: {}", error);
                        return;
                    }
                    runner::run_code(text, module_name);
                } else if args.filename.ends_with(".zenc") {
                    let bytes: Vec<u8>;
                    match fs::read(args.filename) {
                        Err(e) => {
                            println!("read error: {}", e);
                            return;
                        }
                        Ok(data) => {
                            bytes = data;
                        }
                    }
                    runner::run_bytes(bytes);
                }
            } else {
                let mut text = String::new();
                if let Err(error) = file.read_to_string(&mut text) {
                    println!("read error: {}", error);
                    return;
                }
                let filename = format!("{}.zenc", module_name);
                runner::compile_code(text, module_name, filename);
            }
        }
        Err(e) => {
            println!("failed to open {}: {}", args.filename, e);
        }
    }
}
