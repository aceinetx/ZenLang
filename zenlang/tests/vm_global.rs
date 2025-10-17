use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_global_1() {
    let mut tokenizer = Tokenizer::new(
        r#"
let global;

fn x {
	let global = 1;
}
fn main {
	x();
	return global;
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
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_test_global_gc() {
    let mut tokenizer = Tokenizer::new(
        r#"
let global;

fn nop {
    let x = 100;
    while x > 0 {
        let x = x - 1; 
    }
}

fn main {
  let global = [1, 2, 3];
  nop();
	return global[1];
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}
