extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;

use zenlang::compiler::*;
use zenlang::interop::*;
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

    loop {
        if !vm.step() {
            break;
        }
    }

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

    loop {
        if !vm.step() {
            break;
        }
    }

    println!("vm.ret: {}", vm.ret);
    assert_eq!(vm.error, "");
    assert!(
        vm.ret
            .equal(&Value::Object(Rc::new(RefCell::new(object))), &vm)
    );
}

#[test]
fn vm_test_stdlib_array_size() {
    expect_to_return(
        r#"
fn main {
    return array_size([1, 2, 3]);
}
    "#
        .into(),
        Value::Number(3.0),
    );
}

#[test]
fn vm_test_stdlib_array_push() {
    expect_to_return(
        r#"
fn main {
    return array_push([1, 2, 3], 4)[3];
}
    "#
        .into(),
        Value::Number(4.0),
    );
}

#[test]
fn vm_test_stdlib_array_pop() {
    expect_to_return(
        r#"
fn main {
    return array_size(array_pop([1, 2]));
}
    "#
        .into(),
        Value::Number(1.0),
    );
}

#[test]
fn vm_test_stdlib_array_remove() {
    expect_to_return(
        r#"
fn main {
    return array_remove([1, 2, 3], 1)[1];
}
    "#
        .into(),
        Value::Number(3.0),
    );
}

#[test]
fn vm_test_stdlib_array_insert() {
    expect_to_return(
        r#"
fn main {
    return array_insert([1, 3, 4], 1, 2)[1];
}
    "#
        .into(),
        Value::Number(2.0),
    );
}

#[test]
fn vm_test_stdlib_array_count() {
    expect_to_return(
        r#"
fn main {
    return array_count([1,1,1,2,3,4,1], 1);
}
    "#
        .into(),
        Value::Number(4.0),
    );
}

#[test]
fn vm_test_stdlib_array_last() {
    expect_to_return(
        r#"
fn main {
    return array_last([1,1,1,2,3,4]);
}
    "#
        .into(),
        Value::Number(4.0),
    );
}

#[test]
fn vm_test_stdlib_str_split() {
    expect_to_return_obj(
        r#"
fn main {
    return str_split("hello world", " ");
}
    "#
        .into(),
        Object::Array(vec![
            Value::String("hello".into()),
            Value::String("world".into()),
        ]),
    );
}

#[test]
fn vm_test_stdlib_err() {
    expect_to_return(
        r#"
fn main {
    return err(1);
}
    "#
        .into(),
        interop_err(Value::Number(1.0)),
    );
}

#[test]
fn vm_test_stdlib_ok() {
    expect_to_return(
        r#"
fn main {
    return ok(2);
}
    "#
        .into(),
        interop_ok(Value::Number(2.0)),
    );
}

#[test]
fn vm_test_stdlib_get_err() {
    expect_to_return(
        r#"
fn main {
    return get_err(err(2));
}
    "#
        .into(),
        Value::Number(2.0),
    );
}

#[test]
fn vm_test_stdlib_get_ok() {
    expect_to_return(
        r#"
fn main {
    return get_ok(ok(1));
}
    "#
        .into(),
        Value::Number(1.0),
    );
}

#[test]
fn vm_test_stdlib_boolean_false() {
    expect_to_return(
        r#"
fn main {
    return boolean("false");
}
    "#
        .into(),
        Value::Boolean(false),
    );
}

#[test]
fn vm_test_stdlib_boolean_true() {
    expect_to_return(
        r#"
fn main {
    return boolean("true");
}
    "#
        .into(),
        Value::Boolean(true),
    );
}

#[test]
fn vm_test_stdlib_boolean_invalid() {
    expect_to_return(
        r#"
fn main {
    return boolean("1234");
}
    "#
        .into(),
        Value::Null(),
    );
}

#[test]
fn vm_test_stdlib_ord() {
    expect_to_return(
        r#"
fn main {
    return ord("1");
}
    "#
        .into(),
        interop_ok(Value::Number('1' as i64 as f64)),
    );
}

#[test]
fn vm_test_stdlib_chr() {
    expect_to_return(
        r#"
fn main {
    return chr(get_ok(ord("1")));
}
    "#
        .into(),
        Value::String("1".into()),
    );
}

#[test]
fn vm_test_stdlib_stringify() {
    expect_to_return(
        r#"
fn main {
    return stringify([1,2,3]);
}
    "#
        .into(),
        Value::String("[1, 2, 3]".into()),
    );
}

#[test]
fn vm_test_stdlib_number() {
    expect_to_return(
        r#"
fn main {
    return number("123");
}
    "#
        .into(),
        interop_ok(Value::Number(123.0)),
    );
}

#[test]
fn vm_test_stdlib_number_invalid() {
    expect_to_return(
        r#"
fn main {
    return number("123a");
}
    "#
        .into(),
        interop_err(Value::String("invalid float literal".into())),
    );
}

#[test]
fn vm_test_stdlib_clone() {
    expect_to_return(
        r#"
fn main {
    let a1 = [1,2,3];
    let a2 = clone(a1);
    let a1[0] = 0;
    return a2[0];
}
    "#
        .into(),
        Value::Number(1.0),
    );
}

#[test]
fn vm_test_stdlib_file_g_read() {
    expect_to_return(
        r#"
fn main {
    return File.read == read_file;
}
    "#
        .into(),
        Value::Boolean(true),
    );
}

#[test]
fn vm_test_stdlib_file_g_read_bytes() {
    expect_to_return(
        r#"
fn main {
    return File.read_bytes == read_file_bytes;
}
    "#
        .into(),
        Value::Boolean(true),
    );
}

#[test]
fn vm_test_stdlib_file_g_write() {
    expect_to_return(
        r#"
fn main {
    return File.write == write_file;
}
    "#
        .into(),
        Value::Boolean(true),
    );
}

#[test]
fn vm_test_stdlib_file_g_write_bytes() {
    expect_to_return(
        r#"
fn main {
    return File.write_bytes == write_file_bytes;
}
    "#
        .into(),
        Value::Boolean(true),
    );
}
