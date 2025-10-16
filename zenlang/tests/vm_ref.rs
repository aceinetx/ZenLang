use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_ref_array() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn modify arr {
    let arr[0] = 1;
}

fn main {
    let x = [0, 1, 2];
    modify(x);
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
    assert!(vm.objects.is_empty());
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_ref_array_nested() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn modify arr {
    let arr[0] = 1;
}

fn main {
    let x = [0, [0], 2];
    modify(x[1]);
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
    assert!(vm.objects.is_empty());
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_ref_dict() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn modify x {
    let x.num = 69;
}

fn main {
    let dict = {"f" = 1, "t" = 5};
    modify(dict);
    return dict.num;
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
    assert!(vm.objects.is_empty());
    assert!(matches!(vm.ret, Value::Number(69.0)));
}
