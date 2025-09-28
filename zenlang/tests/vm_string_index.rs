use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_string_index_get() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = "ABC";
    return x[0];
}
    "#
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

    println!("{}", vm.ret);
    assert_eq!(vm.error, "");
    if let Value::String(s) = vm.ret {
        assert_eq!(s, "A");
    } else {
        assert!(false);
    }
}

#[test]
fn vm_test_string_index_dotted_get() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = "ABC";
    return x.0;
}
    "#
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

    println!("{}", vm.ret);
    assert_eq!(vm.error, "");
    if let Value::String(s) = vm.ret {
        assert_eq!(s, "A");
    } else {
        assert!(false);
    }
}
