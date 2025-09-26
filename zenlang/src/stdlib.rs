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
        "get_string".into(),
        module.opcodes.len() as u32,
        0,
    ));
    module.opcodes.push(Opcode::Vmcall(3));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "array_size".into(),
        module.opcodes.len() as u32,
        1,
    ));
    module.opcodes.push(Opcode::Vmcall(5));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "array_push".into(),
        module.opcodes.len() as u32,
        2,
    ));
    module.opcodes.push(Opcode::Vmcall(6));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "array_pop".into(),
        module.opcodes.len() as u32,
        1,
    ));
    module.opcodes.push(Opcode::Vmcall(7));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "array_remove".into(),
        module.opcodes.len() as u32,
        2,
    ));
    module.opcodes.push(Opcode::Vmcall(8));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "array_insert".into(),
        module.opcodes.len() as u32,
        3,
    ));
    module.opcodes.push(Opcode::Vmcall(9));
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "str_split".into(),
        module.opcodes.len() as u32,
        2,
    ));
    module.opcodes.push(Opcode::Vmcall(10));
    module.opcodes.push(Opcode::Ret());
    return module.clone();
}
