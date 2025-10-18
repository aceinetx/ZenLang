use zenlang::compiler;
use zenlang::module::Module;
use zenlang::parser;
use zenlang::tokenizer;

#[cfg(target_os = "linux")]
pub fn compile_ffi_module() -> Module {
    let code: String;

    code = String::from(
        r#"
mod stdlib;

let FFI;

fn _ffi_bind_args types args {
    return {
        "_typename" = "FFIArgBinding",
        "_types" = types,
        "_args" = args
    };
}

fn _ffi_load_library name {
    return _vmcall_ret_unsafe_3(name, 1, 70);
}

fn _ffi_unload_library index {
    return _vmcall_ret_unsafe_3(index, 2, 70);
}

fn _ffi_call name arg_binding {
    return _vmcall_ret_unsafe_3(name, arg_binding, 3, 70);
}

fn #[ctor] ffi_init {
    let FFI = {
        "bind_args" = _ffi_bind_args,
        "call" = _ffi_call,
        "Lib" = {
            "load" = _ffi_load_library,
            "unload" = _ffi_unload_library
        }
    };
}
"#,
    );

    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        panic!("ffi compilation failed: {}", e);
    }

    let module = compiler.get_module();
    module.name = "ffi".into();

    return module.clone();
}
