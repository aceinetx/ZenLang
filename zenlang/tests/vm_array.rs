use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_nested_set() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = [0, [0]];
    let x[1][0] = 69;
    return x[1][0];
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
    vm.load_module(module);
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
    assert!(matches!(vm.ret, Value::Number(69.0)));
}
