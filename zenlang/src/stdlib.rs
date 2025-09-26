use crate::{
    module::{Module, ModuleFunction},
    opcode::Opcode,
};

pub fn compile_stdlib_module() -> Module {
    let mut module = Module::new();
    module.name = "stdlib".into();

    module.functions.push(ModuleFunction::new(
        "print".into(),
        module.opcodes.len() as u32,
        1,
    ));
    module.opcodes.push(Opcode::Vmcall(1));
    module.opcodes.push(Opcode::Loadcnu());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "println".into(),
        module.opcodes.len() as u32,
        1,
    ));
    module.opcodes.push(Opcode::Vmcall(2));
    module.opcodes.push(Opcode::Loadcnu());
    module.opcodes.push(Opcode::Ret());

    module.functions.push(ModuleFunction::new(
        "get_string".into(),
        module.opcodes.len() as u32,
        0,
    ));
    module.opcodes.push(Opcode::Vmcall(3));
    module.opcodes.push(Opcode::Ret());
    return module;
}
