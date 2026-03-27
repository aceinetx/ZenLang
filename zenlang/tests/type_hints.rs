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

/*
#[test]
fn tokenizer_test_number() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn add x: num y: num {
}

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
*/
