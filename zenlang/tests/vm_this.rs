extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;

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

    let _ = vm.load_module(&zenlang::stdlib::compile_stdlib_module());
    let _ = vm.load_module(module);

    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    vm.run_until_halt();

    println!("vm.ret: {:?}", vm.ret);
    assert_eq!(vm.error, "");
    assert!(vm.ret.equal(&value, &vm));
}

fn expect_to_return_obj(code: String, object: Object) {
    let mut tokenizer = Tokenizer::new(code);
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module.opcodes);

    let _ = vm.load_module(&zenlang::stdlib::compile_stdlib_module());
    let _ = vm.load_module(module);

    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    vm.run_until_halt();

    println!("vm.ret: {}", vm.ret);
    assert_eq!(vm.error, "");
    assert!(
        vm.ret
            .equal(&Value::Object(Rc::new(RefCell::new(object))), &vm)
    );
}

#[test]
fn vm_test_self_1() {
    expect_to_return(
        r#"
fn f2 {
    let self.hi = self.hi + 3;
}

fn f {
    let self.hi = 1;
    self.test2();
}

fn main {
    let obj = {
        "test" = f,
        "test2" = f2,
    };
    obj.test();
    return obj.hi;
} "#
        .into(),
        Value::Number(4.0),
    );
}

#[test]
fn vm_test_self_2() {
    expect_to_return(
        r#"
fn f2 {
    let self.hi = 3;
}

fn f {
    let self.hi = 1;
    let obj = {
        "test" = f2
    };
    obj.test();
    let self.hi = obj.hi - 1;
}

fn main {
    let obj = {
        "test" = f,
    };
    obj.test();
    return obj.hi;
} "#
        .into(),
        Value::Number(2.0),
    );
}
