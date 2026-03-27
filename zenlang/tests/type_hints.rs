use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn type_hints_test_compile() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn f x: num y: str {
}
"#
        .into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let module = compiler.get_module();
    let func = &module.functions[0];
    assert_eq!(func.args_type_hints[0], "num");
    assert_eq!(func.args_type_hints[1], "str");
}

#[test]
fn type_hints_test_fail() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn f x: number y: string {
}

fn main {
    f(0.0, 1);
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
    assert_eq!(
        vm.error,
        "call: expected string as a 1 argument but found number"
    );
}

#[test]
fn type_hints_test_pass() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn add x: number y: number {
    return x + y;
}

fn main {
    return add(3, 2);
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
    assert!(matches!(vm.ret, Value::Number(5.0)));
}

#[test]
fn type_hints_test_result() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn f x: Result {
}

fn main {
    f({"_typename" = "Result"});
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
    println!("{:?}", vm.stack);
    assert_eq!(vm.error, "");
}
