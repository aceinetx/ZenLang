use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_bitor() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 | 5 + 2; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(7.0)));
}

#[test]
fn vm_test_bitand() {
    let mut tokenizer = Tokenizer::new("fn main {return 2 + 1 & 5 + 2; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(3.0)));
}

#[test]
fn vm_test_bitshr() {
    let mut tokenizer = Tokenizer::new("fn main {return 120 + 3 >> 1 + 1; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(30.0)));
}

#[test]
fn vm_test_bitshl() {
    let mut tokenizer = Tokenizer::new("fn main {return 120 + 3 << 1 + 1; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(492.0)));
}
