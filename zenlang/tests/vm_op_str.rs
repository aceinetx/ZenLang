use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_op_str_add() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    return "a" + "b" + "c";
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

    if let Value::String(str) = vm.ret {
        assert_eq!(str, "abc");
    } else {
        assert_eq!("vm.ret is not a string", "");
    }
}

#[test]
fn vm_op_str_mul() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    return "a" * 3;
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

    if let Value::String(str) = vm.ret {
        assert_eq!(str, "aaa");
    } else {
        assert_eq!("vm.ret is not a string", "");
    }
}
