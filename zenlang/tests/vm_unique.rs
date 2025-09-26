//! tests vm_unique
//!
//! These tests test unique edge cases
use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_unique_1() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let self = {"data" = []};
    let self.data[0] = 1;
    let self.data[1] = 2;
    return self.data[1];
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
fn vm_test_unique_2() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let self = {"data" = []};
    let self["data"][0] = 1;
    let self["data"][1] = 2;
    return self.data[1];
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}

#[test]
fn vm_test_unique_3() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let self = [{"data" = 0}];
    let self[0]["data"] = 1;
    let self[0]["data"] = 2;
    return self.0["data"];
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}

#[test]
fn vm_test_unique_4() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let self = [{"data" = 0}];
    let self.0["data"] = 1;
    let self.0["data"] = 2;
    return self[0]["data"];
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}
