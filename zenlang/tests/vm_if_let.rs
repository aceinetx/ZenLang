use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_if_let() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    if let x = null {
        return 1;
    }
    return 0;
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
    if let Err(e) = vm.load_module(module) {
        assert_eq!(e, "");
    }
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
fn vm_test_if_let_true() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    if let x = 1 {
        return 1;
    }
    return 0;
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
    if let Err(e) = vm.load_module(module) {
        assert_eq!(e, "");
    }
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_test_elif_let() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = null;
    let y = 1;
    if let _ = x {
        return 1;
    } elif let _ = y {
        return 2;
    }
    return 0;
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
    if let Err(e) = vm.load_module(module) {
        assert_eq!(e, "");
    }
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
fn vm_test_elif_let_2() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 1;
    let y = null;
    if let _ = x {
        return 1;
    } elif let _ = y {
        return 2;
    }
    return 0;
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
    if let Err(e) = vm.load_module(module) {
        assert_eq!(e, "");
    }
    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    loop {
        if !vm.step() {
            break;
        }
    }

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(1.0)));
}
