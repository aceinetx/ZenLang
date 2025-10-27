extern crate alloc;

use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

fn expect_to_return(code: String, value: Value) {
    let mut tokenizer = Tokenizer::new(code);
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

    vm.run_until_halt();

    println!("vm.ret: {:?}", vm.ret);
    assert_eq!(vm.error, "");
    assert!(vm.ret.equal(&value, &vm));
}

#[test]
fn vm_test_environ_1() {
    expect_to_return(
        r#"
fn f {
    let x = 5;
    return null;
} 

fn main {
    f();
    return x;
}
    "#
        .into(),
        Value::Null(),
    );
}

#[test]
fn vm_test_environ_2() {
    expect_to_return(
        r#"
fn f {
    let x = 5;
    return null;
} 

fn main {
    let x = 3;
    f();
    return x;
}
    "#
        .into(),
        Value::Number(3.0),
    );
}
