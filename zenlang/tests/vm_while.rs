use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_while() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 2; 
    let y = 0; 
    while x > 0 {
        let x = x - 1; 
        let y = y + 1;
    } 

    return y;
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
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}

#[test]
fn vm_test_while_2() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 0; 
    let y = 0; 
    while x > 0 {
        let y = 2;
    } 

    return y;
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
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(0.0)));
}

#[test]
fn vm_test_while_3() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 5; 
    let y = 0; 
    while x > 0 {
        let x = x - 1; 
        let y = y + 1;
        if y == 3 {
            break;
        }
    } 

    return y;
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
    println!("{:?}", module);
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
