use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_dict_get() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 42, "b" = 69};
    return x["a"] + x["b"];
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_set() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 0, "b" = 69};
    let x["a"] = 42;
    return x["a"] + x["b"];
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_get_nested() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = {"v" = 69}, "b" = 42};
    return x["a"]["v"] + x["b"];
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_set_nested() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = {"v" = 0}, "b" = 42};
    let x["a"]["v"] = 69;
    return x["a"]["v"] + x["b"];
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

// * Dotted

#[test]
fn vm_test_dict_dotted_get() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 42, "b" = 69};
    return x.a + x.b;
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_dotted_set() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = 0, "b" = 69};
    let x.a = 42;
    return x.a + x.b;
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_dotted_get_nested() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = {"v" = 69}, "b" = 42};
    return x.a.v + x.b;
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}

#[test]
fn vm_test_dict_dotted_set_nested() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main { 
    let x = {"a" = {"v" = 0}, "b" = 42};
    let x.a.v = 69;
    return x.a.v + x.b;
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

    assert_eq!(vm.error, "");
    assert!(matches!(vm.ret, Value::Number(111.0)));
}
