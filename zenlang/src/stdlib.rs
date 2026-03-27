use crate::{
    compiler,
    module::{Module, ModuleFunction},
    opcode::Opcode,
    parser, tokenizer,
};
use alloc::{string::*, vec::Vec};

pub fn compile_stdlib_module() -> Module {
    let code = String::from(
        r#"
let File;

fn #[naked] print str: string {
    vmcall 1;
    return null;
} 
fn #[naked] println str: string {
    vmcall 2;
    return null;
} 
fn get_string {
    return _vmcall_ret_unsafe_1(3);
}
fn array_size array: array {
    return _vmcall_ret_unsafe_2(array, 5);
}
fn array_push array: array element {
    return _vmcall_ret_unsafe_3(array, element, 6);
}
fn array_pop array: array {
    return _vmcall_ret_unsafe_2(array, 7);
}
fn array_remove array: array index: number {
    return _vmcall_ret_unsafe_3(array, index, 8);
}
fn array_insert array: array index: number element {
    return _vmcall_ret_unsafe_4(array, index, element, 9);
}
fn array_count array: array element {
    let i = 0;
    let count = 0;
    let size = array_size(array);
    while i < size {
        if array[i] == element {
            let count = count + 1;
        }
        
        let i = i + 1;
    }
    return count;
}
fn array_last array: array {
    return array[array_size(array) - 1];
}
fn str_split str: string delimiter: string {
    return _vmcall_ret_unsafe_3(str, delimiter, 10);
}
fn err value {
    return {"_typename" = "Result", "_err" = value, "_ok" = null};
}
fn ok value {
    return {"_typename" = "Result", "_err" = null, "_ok" = value};
}
fn get_err result: Result {
    return result._err;
}
fn get_ok result: Result {
    return result._ok;
}
fn read_file_bytes path: string {
    if let bytes = _vmcall_ret_unsafe_2(path, 11){
        return ok(bytes);
    }
    return err("Read failed");
}
fn read_file path: string {
    if let bytes = _vmcall_ret_unsafe_2(path, 12){
        return ok(bytes);
    }
    return err("Read failed");
}
fn #[naked] write_file_bytes path: string bytes: array {
    vmcall 13;
    return null;
}
fn #[naked] write_file path: string bytes: array {
    vmcall 14;
    return null;
}
fn boolean str: string {
    if str == "true" {
        return true;
    } elif str == "false" {
        return false;
    }
    return null;
}
fn ord ch: string {
    if ch == "" {
        return err("ch is empty");
    }

    return ok(_vmcall_ret_unsafe_2(ch, 15));
}
fn chr ch: number {
    return _vmcall_ret_unsafe_2(ch, 16);
}
fn stringify any {
    return _vmcall_ret_unsafe_2(any, 17);
}
fn number str: string {
    return _vmcall_ret_unsafe_2(str, 18);
}
fn clone obj {
    return _vmcall_ret_unsafe_2(obj, 19);
}
fn #[ctor] stdlib_init {
    let File = {
        "read" = read_file,
        "read_bytes" = read_file_bytes,
        "write" = write_file, 
        "write_bytes" = write_file_bytes,
    };
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
        Vec::new(),
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_2".into(),
        module.opcodes.len() as u32,
        2,
        Vec::new(),
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_3".into(),
        module.opcodes.len() as u32,
        3,
        Vec::new(),
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "_vmcall_ret_unsafe_4".into(),
        module.opcodes.len() as u32,
        4,
        Vec::new(),
        false,
    ));
    module.opcodes.push(Opcode::Dynvmcall());
    module.opcodes.push(Opcode::Ret());

    return module.clone();
}
