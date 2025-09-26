use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_ret_if_false() {
    let mut tokenizer =
        Tokenizer::new("fn main {let x = 2; if x == 3 {return 1;} return 0; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(0.0)));
}

#[test]
fn vm_test_ret_if_true() {
    let mut tokenizer =
        Tokenizer::new("fn main {let x = 2; if x == 2 {return 1;} return 0; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_test_ret_if_else() {
    let mut tokenizer = Tokenizer::new(
        "fn main {let x = 2; if x == 3 {return 1;} else {return 2;} return 0; }".into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module);
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
    println!("{}", vm.ret);
    assert!(matches!(vm.ret, Value::Number(2.0)));
}

#[test]
fn vm_test_ret_if_elif() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 2; 
    if x == 3 {
        return 1;
    } elif x == 2 {
        return 5;
    } else {
        return 2;
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
    assert!(matches!(vm.ret, Value::Number(5.0)));
}

#[test]
fn vm_test_ret_if_chained_false() {
    let mut tokenizer = Tokenizer::new(
        "fn main {let x = 2; let y = 5; if x == 2 {if y == 3 {return 1;}} return 0; }".into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(0.0)));
}

#[test]
fn vm_test_ret_if_chained_true() {
    let mut tokenizer = Tokenizer::new(
        "fn main {let x = 2; let y = 3; if x == 2 {if y == 3 {return 1;}} return 0; }".into(),
    );
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    println!("{:?}", module);
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
    assert!(matches!(vm.ret, Value::Number(1.0)));
}

#[test]
fn vm_test_ret_if_chained_elif() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 2; 
    let y = 3; 
    if x == 2 {
        if y == 2 {
            return 1;
        } elif y == 1 {
            return 2;
        } else {
            return 5;
        }
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
    assert!(matches!(vm.ret, Value::Number(5.0)));
}

#[test]
fn vm_test_ret_if_chained_elif_2() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 2; 
    let y = 3; 
    if x == 2 {
        if y == 2 {
            return 1;
        } elif y == 3 {
            return 2;
        } else {
            return 5;
        }
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
    assert!(matches!(vm.ret, Value::Number(2.0)));
}

#[test]
fn vm_test_ret_if_chained_elif_3() {
    let mut tokenizer = Tokenizer::new(
        r#"
fn main {
    let x = 2; 
    let y = 3; 
    if x == 1 {
        return 1;
    } elif x == 2 {
        if y == 1 {
            return 2;
        } elif y == 3 {
            return 3;
        } else {
            return 5;
        }
    } else {
        return 4;
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
    assert!(matches!(vm.ret, Value::Number(3.0)));
}
