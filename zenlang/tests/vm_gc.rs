use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::vm::*;

#[test]
fn vm_test_gc_1() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 42, "b" = 69};
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
    assert!(vm.allocated_objs.is_empty());
}

#[test]
fn vm_test_gc_2() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 42, "b" = 69};
    return null;
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

    assert!(vm.allocated_objs.is_empty());
}

#[test]
fn vm_test_gc_3() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 42, "b" = 69};
    let y = x;
    return null;
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

    assert!(vm.allocated_objs.is_empty());
}
