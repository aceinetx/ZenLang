extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;
use zenlang::state::State;

use zenlang::compiler::*;
use zenlang::interop::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;
use zenlang::value::*;
use zenlang::vm::*;

pub fn expect_to_return(code: String, value: Value) {
    let mut tokenizer = Tokenizer::new(code);
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    let mut state = State::new();
    let mut global_state = state.global_state.borrow_mut();

    let module = compiler.get_module();
    println!("{:?}", module.opcodes);

    let _ = global_state.load_module(&zenlang::stdlib::compile_stdlib_module());
    let _ = global_state.load_module(module);

    if let Err(e) = vm.set_entry_function("main") {
        assert_eq!(e, "");
    }

    state.run_until_halt().unwrap();
    let ret = core::mem::take(&mut state.vms[0].ret);

    println!("vm.ret: {:?}", ret);
    assert!(ret.equal(&value));
}

pub fn expect_to_return_obj(code: String, object: Object) {
    expect_to_return(code, Value::Object(Rc::new(RefCell::new(object))));
}
