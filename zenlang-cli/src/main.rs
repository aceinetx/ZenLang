use std::{env, fs, io::Read};
use zenlang::{compiler, parser, stdlib, strong_u64::U64BitsControl, tokenizer, vm};

mod platform;

fn run_code(code: String) {
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);

    if let Err(e) = compiler.compile() {
        println!("compile error: {}", e);
        return;
    }

    let module = compiler.get_module();
    //println!("{:?}", module);

    let mut vm = vm::VM::new();
    vm.platform = Some(Box::new(platform::Platform::new()));

    let mut stdlib = stdlib::compile_stdlib_module();
    vm.load_module(&mut stdlib);
    vm.load_module(module);
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("zenlang: no filename provided");
        return;
    }

    match fs::File::open(&args[1]) {
        Ok(mut file) => {
            let mut text = String::new();
            if let Err(error) = file.read_to_string(&mut text) {
                println!("read error: {}", error);
                return;
            }
            run_code(text);
        }
        Err(e) => {
            println!("failed to open {}: {}", &args[1], e);
        }
    }
}
