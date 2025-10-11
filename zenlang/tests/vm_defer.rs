use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_defer_block() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    {
        let x = 0;
        defer {
            let x = 5;
        };
        let x = 1;
    }
    return x;
}"#
        .into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    let _ = vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    println!("{}", vm.ret);
    assert!(matches!(vm.ret, Value::Number(5.0)));
}

#[test]
fn vm_test_defer_oneline() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    {
        let x = 0;
        defer let x = 5;
        let x = 1;
    }
    return x;
}"#
        .into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);
    let _ = vm.load_module(module);
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    println!("{}", vm.ret);
    assert!(matches!(vm.ret, Value::Number(5.0)));
}
