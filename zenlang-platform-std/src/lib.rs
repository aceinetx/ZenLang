//! ZenLang Platform implementation for rust's standard library
pub mod ffi;
use std::fs;
use zenlang::module::Module;
use zenlang::platform;
use zenlang::stdlib::compile_stdlib_module;

#[cfg(target_os = "linux")]
use dlopen::raw::*;

use crate::ffi::compile_ffi_module;
use zenlang::vm::VM;

pub struct Platform {
    #[cfg(target_os = "linux")]
    pub(crate) ffi_handles: Vec<Box<Library>>,
    #[cfg(not(target_os = "linux"))]
    pub(crate) ffi_handles: Vec<i8>,
}

impl Platform {
    pub fn new() -> Self {
        return Self {
            ffi_handles: Vec::new(),
        };
    }
}

#[cfg(target_os = "linux")]
pub fn vmcall_70(platform: &mut Platform, vm: &mut VM) {
    use zenlang::interop::*;
    use zenlang::value::Object;
    use zenlang::value::Value;

    let operation: u8;
    if let Value::Number(num) = vm.stack.pop().unwrap() {
        operation = num as i64 as u8;
    } else {
        vm.error = format!("expected number as the operation");
        return;
    }

    match operation {
        1 => {
            if let Value::String(name) = vm.stack.pop().unwrap() {
                match Library::open(name) {
                    Ok(lib) => {
                        let index = platform.ffi_handles.len();
                        let lib = Box::new(lib);

                        platform.ffi_handles.push(lib);

                        vm.stack.push(interop_ok(Value::Number(index as f64)));
                    }
                    Err(error) => vm.stack.push(interop_err(Value::String(format!(
                        "failed to load lib: {}",
                        error
                    )))),
                }
            } else {
                vm.error = format!("expected string as the library name");
                return;
            }
        }
        2 => {
            if let Value::Number(index) = vm.stack.pop().unwrap() {
                let index = index as usize;
                if index >= platform.ffi_handles.len() {
                    return;
                }

                drop(platform.ffi_handles.remove(index));
            } else {
                vm.error = format!("expected number as library index");
                return;
            }
        }
        3 => {
            match (vm.stack.pop().unwrap(), vm.stack.pop().unwrap()) {
                (Value::String(name), Value::Object(bind_args)) => match &*bind_args.borrow() {
                    Object::Dictionary(bind_args) => {
                        let needed_keys: Vec<String> =
                            vec!["_typename".into(), "_types".into(), "_args".into()];
                        for key in needed_keys.iter() {
                            if !bind_args.contains_key(key) {
                                vm.error = format!(
                                    "second argument is not of type FFIArgBinding (ffi vmcall operation 3: call func)"
                                );
                                return;
                            }
                        }

                        if !bind_args
                            .get("_typename")
                            .unwrap()
                            .equal(&Value::String("FFIArgBinding".into()), vm)
                        {
                            vm.error = format!(
                                "second argument is not of type FFIArgBinding (ffi vmcall operation 3: call func)"
                            );
                            return;
                        }

                        match (
                            bind_args.get("_types").unwrap(),
                            bind_args.get("_args").unwrap(),
                        ) {
                            (Value::Object(types), Value::Object(args)) => {
                                match (&*types.borrow(), &*args.borrow()) {
                                    (Object::Array(types), Object::Array(args)) => {
                                        if args.len() > 5 || types.len() > 5 {
                                            vm.error = format!(
                                                "only up to 5 args and types are supported"
                                            );
                                            return;
                                        }

                                        return;
                                    }
                                    _ => {
                                        vm.error = format!(
                                            "second argument is not of type FFIArgBinding (ffi vmcall operation 3: call func)"
                                        );
                                        return;
                                    }
                                }
                            }
                            _ => {
                                vm.error = format!(
                                    "second argument is not of type FFIArgBinding (ffi vmcall operation 3: call func)"
                                );
                                return;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            vm.error = format!(
                "expected these arguments: string (func name), FFIArgBinding (ffi vmcall operation 3: call func)"
            );
        }
        _ => {
            vm.error = format!("unknown ffi operation under index {}", operation);
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn vmcall_70(_platform: &mut Platform, _vm: &mut VM) {}

impl platform::Platform for Platform {
    fn print(&self, s: String) {
        print!("{}", s);
    }

    fn get_string(&self) -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {}
        }
        input.trim().to_string()
    }

    fn get_module(&self, name: String) -> Option<Module> {
        if name == "stdlib" {
            let module = compile_stdlib_module();
            return Some(module);
        } else if name == "ffi" {
            if std::env::consts::OS == "linux" {
                let module = compile_ffi_module();
                return Some(module);
            }
        } else {
            let filename = name + ".zenc";
            if let Some(bytes) = self.read_file_bytes(filename) {
                let mut module = Module::new();
                if module.load(bytes).is_err() {
                    return None;
                }
                return Some(module);
            }
        }
        return None;
    }

    fn read_file_bytes(&self, name: String) -> Option<Vec<u8>> {
        match fs::read(name) {
            Err(_) => {
                return None;
            }
            Ok(bytes) => {
                return Some(bytes);
            }
        }
    }

    fn write_file_bytes(&self, name: String, bytes: Vec<u8>) {
        let _ = fs::write(name, bytes);
    }

    fn vmcall(&mut self, vm: &mut zenlang::vm::VM, index: u8) -> bool {
        match index {
            70 => {
                vmcall_70(self, vm);
                return true;
            }
            _ => return false,
        }
    }
}
