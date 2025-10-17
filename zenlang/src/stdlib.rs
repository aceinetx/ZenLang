use crate::{
    compiler,
    module::{Module, ModuleFunction},
    opcode::Opcode,
    parser, tokenizer,
};
use alloc::string::*;

pub fn compile_stdlib_module() -> Module {
    let code = String::from(
        r#"
fn #[naked] print str {
    vmcall 1;
    return null;
} 
fn #[naked] println str {
    vmcall 2;
    return null;
} 
fn get_string {
    return _vmcall_ret_unsafe_1(3);
}
fn array_size array {
    return _vmcall_ret_unsafe_2(array, 5);
}
fn array_push array element {
    return _vmcall_ret_unsafe_3(array, element, 6);
}
fn array_pop array {
    return _vmcall_ret_unsafe_2(array, 7);
}
fn array_remove array index {
    return _vmcall_ret_unsafe_3(array, index, 8);
}
fn array_insert array index element {
    return _vmcall_ret_unsafe_4(array, index, element, 9);
}
fn str_split str delimiter {
    return _vmcall_ret_unsafe_3(str, delimiter, 10);
}
fn err value {
    return {"_err" = value, "_ok" = null};
}
fn ok value {
    return {"_err" = null, "_ok" = value};
}
fn get_err result {
    return result._err;
}
fn get_ok result {
    return result._ok;
}
fn array_last array {
    return array[array_size(array) - 1];
}
fn read_file_bytes path {
    if let bytes = _vmcall_ret_unsafe_2(path, 11){
        return ok(bytes);
    }
    return err("Read failed");
}
fn read_file path {
    if let bytes = _vmcall_ret_unsafe_2(path, 12){
        return ok(bytes);
    }
    return err("Read failed");
}
fn #[naked] write_file_bytes path bytes {
    vmcall 13;
    return null;
}
fn #[naked] write_file path bytes {
    vmcall 14;
    return null;
}
fn boolean str {
    if str == "true" {
        return true;
    } elif str == "false" {
        return false;
    }
    return null;
}
fn ord ch {
    if ch == "" {
        return err("ch is empty");
    }

    return ok(_vmcall_ret_unsafe_2(ch, 15));
}
fn chr ch {
    return _vmcall_ret_unsafe_2(ch, 16);
}
fn stringify any {
    return _vmcall_ret_unsafe_2(any, 17);
}
fn number str {
    return _vmcall_ret_unsafe_2(str, 18);
}
    "#,
    );
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        panic!("stdlib compilation failed: {}", e);
    }

    let module = compiler.get_module();
    module.name = "stdlib".into();

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_1".into(),
        module.opcodes.len() as u32,
        1,
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_2".into(),
        module.opcodes.len() as u32,
        2,
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_3".into(),
        module.opcodes.len() as u32,
        3,
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_4".into(),
        module.opcodes.len() as u32,
        4,
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    return module.clone();
}
