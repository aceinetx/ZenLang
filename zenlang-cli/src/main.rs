use std::{
    env,
    fs::{self},
    io::Read,
};
use zenlang::{
    compiler, module, parser::parser, stdlib, strong_u64::U64BitsControl, tokenizer, vm,
};

mod platform;

fn run_vm(vm: &mut vm::VM) {
    vm.platform = Some(Box::new(platform::Platform::new()));

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

fn run_code(code: String) {
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
    //println!("{:?}", module);

    let mut vm = vm::VM::new();

    let mut stdlib = stdlib::compile_stdlib_module();
    vm.load_module(&mut stdlib);
    vm.load_module(module);
    run_vm(&mut vm);
}

fn compile_code(code: String, out_filename: String) {
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

    let mut stdlib = stdlib::compile_stdlib_module();
    vm.load_module(&mut stdlib);
    vm.load_module(&mut module);

    run_vm(&mut vm);
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

    match fs::File::open(&args[1]) {
        Ok(mut file) => {
            if !compile {
                if args[1].ends_with(".zen") {
                    let mut text = String::new();
                    if let Err(error) = file.read_to_string(&mut text) {
                        println!("read error: {}", error);
                        return;
                    }
                    run_code(text);
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
                compile_code(text, "a.zenc".into());
            }
        }
        Err(e) => {
            println!("failed to open {}: {}", &args[1], e);
        }
    }
}
