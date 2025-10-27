use std::fs;
use zenlang::{compiler, module, parser, strong_u64::U64BitsControl, tokenizer, vm};
use zenlang_platform_std::*;

fn run_vm(vm: &mut vm::VM) {
    if let Err(e) = vm.set_entry_function("main") {
        println!("vm error: {}", e);
        return;
    }
    //println!("{:?}", vm.modules);

    vm.run_until_halt();

    if !vm.error.is_empty() {
        let mut pc = vm.pc;
        pc.sub_low(1);

        println!("\n-- begin runtime error --");
        println!("{}", vm.error);
        if let Some(name) = vm.get_function_name_from_pc(pc) {
            println!("runtime error in function {}", name,);
        }
        println!("runtime error at pc = {}:{}", pc.get_low(), pc.get_high(),);
        println!("-- end runtime error --");
        return;
    }

    println!("returned {}", vm.ret);

    if !vm.stack.is_empty() {
        println!("{} values remained on stack!", vm.stack.len());
    } else {
        println!("no values leaked on stack");
    }

    // There will always be at least 1 environ for the main function that doesn't get freed
    // That's normal
    if vm.environs.len() > 1 {
        println!("{} environs remained!", vm.environs.len());
    } else {
        println!("no environs leaked");
    }
}

pub fn run_code(code: String, module_name: String) {
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

pub fn compile_code(code: String, module_name: String, out_filename: String) {
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

pub fn run_bytes(bytes: Vec<u8>) {
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
