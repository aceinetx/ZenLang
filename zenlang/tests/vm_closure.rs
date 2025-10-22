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
    println!("{}", module.format_debug_opcodes());

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
fn vm_test_closure_1() {
    expect_to_return(
        r#"
fn call f n {
    let x = 4;
    f(n);
    return null;
} 

fn main {
    let num = 5;
    let closure = fn n {
        let num = n;
        if x != null {
            let num = 3;
        }
        return null;
    };
    call(closure, 6);
    return num;
}
    "#
        .into(),
        Value::Number(6.0),
    );
}

#[test]
fn vm_test_closure_2() {
    expect_to_return(
        r#"
fn call f n {
    let x = 4;
    f(n);
    return x;
} 

fn main {
    let num = 5;
    let closure = fn n {
        let num = n;
        let x = 0;
        if x != null {
            let num = 3;
        }
        return null;
    };
    if call(closure, 6) == 0 {
        return null;
    }
    return num;
}
    "#
        .into(),
        Value::Number(3.0),
    );
}
