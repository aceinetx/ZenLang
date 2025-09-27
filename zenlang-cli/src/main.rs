use std::{
    env,
    fs::{self},
    io::Read,
    path::Path,
};
use zenlang::{compiler, module, parser, strong_u64::U64BitsControl, tokenizer, vm};
use zenlang_platform_std::*;

fn run_vm(vm: &mut vm::VM) {
    if let Err(e) = vm.set_entry_function("main") {
        println!("vm error: {}", e);
        return;
    }

    loop {
        if !vm.step() {
            break;
        }
    }
    if !vm.error.is_empty() {
        println!("\n-- begin runtime error --");
        println!("{}", vm.error);
        println!(
            "runtime error at pc = {}:{}",
            vm.pc.get_low() - 1,
            vm.pc.get_high(),
        );
        println!("-- end runtime error --");
        return;
    }
    println!("returned {}", vm.ret);
}

fn run_code(code: String, module_name: String) {
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);

    if let Err(e) = compiler.compile() {
        println!("compile error: {}", e);
        return;
    }

    if compiler.warnings.len() > 0 {
        println!("compile warnings:");
        for warning in compiler.warnings.iter() {
            println!("- {}", warning);
        }
    }

    let module = compiler.get_module();
    module.name = module_name;

    let mut vm = vm::VM::new();
    vm.platform = Some(Box::new(Platform::new()));

    if let Err(e) = vm.load_module(&module) {
        println!("{}", e);
        return;
    }

    run_vm(&mut vm);
}

fn compile_code(code: String, module_name: String, out_filename: String) {
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);

    if let Err(e) = compiler.compile() {
        println!("compile error: {}", e);
        return;
    }

    if compiler.warnings.len() > 0 {
        println!("compile warnings:");
        for warning in compiler.warnings.iter() {
            println!("- {}", warning);
        }
    }

    let module = compiler.get_module();
    module.name = module_name;
    match module.compile() {
        Err(e) => {
            println!("module compile error: {}", e);
        }
        Ok(bytes) => {
            let _ = fs::write(out_filename, bytes);
        }
    }
}

fn run_bytes(bytes: Vec<u8>) {
    let mut module = module::Module::new();
    if let Err(e) = module.load(bytes) {
        println!("load error: {}", e);
        return;
    }

    let mut vm = vm::VM::new();
    vm.platform = Some(Box::new(Platform::new()));

    if let Err(e) = vm.load_module(&module) {
        println!("{}", e);
        return;
    }

    run_vm(&mut vm);
}

fn get_module_name_from_path(path: String) -> String {
    let path = Path::new(&path);

    if let Some(stem) = path.file_stem() {
        let filename_without_extension = stem.to_string_lossy();
        return filename_without_extension.to_string();
    }
    return path.to_string_lossy().to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("zenlang: no filename provided");
        return;
    }

    let mut compile: bool = false;
    if args.len() >= 3 {
        if args[2] == "compile" {
            compile = true;
        }
    }

    let module_name = get_module_name_from_path(args[1].clone());
    match fs::File::open(&args[1]) {
        Ok(mut file) => {
            if !compile {
                if args[1].ends_with(".zen") {
                    let mut text = String::new();
                    if let Err(error) = file.read_to_string(&mut text) {
                        println!("read error: {}", error);
                        return;
                    }
                    run_code(text, module_name);
                } else if args[1].ends_with(".zenc") {
                    let bytes: Vec<u8>;
                    match fs::read(&args[1]) {
                        Err(e) => {
                            println!("read error: {}", e);
                            return;
                        }
                        Ok(data) => {
                            bytes = data;
                        }
                    }
                    run_bytes(bytes);
                }
            } else {
                let mut text = String::new();
                if let Err(error) = file.read_to_string(&mut text) {
                    println!("read error: {}", error);
                    return;
                }
                compile_code(text, module_name, "a.zenc".into());
            }
        }
        Err(e) => {
            println!("failed to open {}: {}", &args[1], e);
        }
    }
}
