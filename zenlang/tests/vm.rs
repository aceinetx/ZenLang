use zenlang::compiler::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

#[test]
fn vm_test_ret_num() {
    let mut tokenizer = Tokenizer::new("fn main {return 1.23;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    vm.load_module(module);

    loop {
        if !vm.step() {
            break;
        }
    }

    assert!(vm.error.is_empty());
    assert!(matches!(vm.ret, Value::Number(1.23)));
}

#[test]
fn vm_test_ret_string() {
    let mut tokenizer = Tokenizer::new("fn main {return \"Hello\";}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    vm.load_module(module);

    loop {
        if !vm.step() {
            break;
        }
    }

    assert!(vm.error.is_empty());
    if let Value::String(s) = vm.ret {
        assert_eq!(s, "Hello");
    } else {
        assert!(false);
    }
}

#[test]
fn vm_test_ret_true() {
    let mut tokenizer = Tokenizer::new("fn main {return true;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    vm.load_module(module);

    loop {
        if !vm.step() {
            break;
        }
    }

    assert!(vm.error.is_empty());
    assert!(matches!(vm.ret, Value::Boolean(true)));
}

#[test]
fn vm_test_ret_false() {
    let mut tokenizer = Tokenizer::new("fn main {return false;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    vm.load_module(module);

    loop {
        if !vm.step() {
            break;
        }
    }

    assert!(vm.error.is_empty());
    assert!(matches!(vm.ret, Value::Boolean(false)));
}

#[test]
fn vm_test_ret_null() {
    let mut tokenizer = Tokenizer::new("fn main {return null;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut vm = VM::new();
    let module = compiler.get_module();
    vm.load_module(module);

    loop {
        if !vm.step() {
            break;
        }
    }

    assert!(vm.error.is_empty());
    assert!(matches!(vm.ret, Value::Null()));
}
