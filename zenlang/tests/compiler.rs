use zenlang::compiler::*;
use zenlang::opcode::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;

#[test]
fn compiler_test_func_1() {
    let mut tokenizer = Tokenizer::new("fn main {}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 2);
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_func_2() {
    let mut tokenizer = Tokenizer::new("fn add x y {} fn main {}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.functions.len(), 2);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "add");
    assert_eq!(module.functions[0].args_count, 2);
    assert_eq!(module.functions[1].addr, 4);
    assert_eq!(module.functions[1].name, "main");
    assert_eq!(module.functions[1].args_count, 0);
}

#[test]
fn compiler_test_return() {
    let mut tokenizer = Tokenizer::new("fn main {return null;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 2);
    assert!(matches!(module.opcodes[0], Opcode::LoadNull()));
    assert!(matches!(module.opcodes[1], Opcode::Ret()));
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_return_add() {
    let mut tokenizer = Tokenizer::new("fn main {return 2+3;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 4);
    assert!(matches!(module.opcodes[0], Opcode::LoadConstant(2.0)));
    assert!(matches!(module.opcodes[1], Opcode::LoadConstant(3.0)));
    assert!(matches!(module.opcodes[2], Opcode::Add()));
    assert!(matches!(module.opcodes[3], Opcode::Ret()));
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_let() {
    let mut tokenizer = Tokenizer::new("fn main {let x = 1+2*3;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 8);
    assert!(matches!(module.opcodes[0], Opcode::LoadConstant(1.0)));
    assert!(matches!(module.opcodes[1], Opcode::LoadConstant(2.0)));
    assert!(matches!(module.opcodes[2], Opcode::LoadConstant(3.0)));
    assert!(matches!(module.opcodes[3], Opcode::Mul()));
    assert!(matches!(module.opcodes[4], Opcode::Add()));
    if let Opcode::StoreVar(s) = &module.opcodes[5] {
        assert_eq!(s.to_string(), "x");
    } else {
        assert!(false);
    }
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_varref() {
    let mut tokenizer = Tokenizer::new("fn main {let x = 1+2*3;return x;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 8);
    println!("{:?}", module.opcodes);
    assert!(matches!(module.opcodes[0], Opcode::LoadConstant(1.0)));
    assert!(matches!(module.opcodes[1], Opcode::LoadConstant(2.0)));
    assert!(matches!(module.opcodes[2], Opcode::LoadConstant(3.0)));
    assert!(matches!(module.opcodes[3], Opcode::Mul()));
    assert!(matches!(module.opcodes[4], Opcode::Add()));
    if let Opcode::StoreVar(s) = &module.opcodes[5] {
        assert_eq!(s.to_string(), "x");
    } else {
        assert!(false);
    }
    if let Opcode::LoadVar(s) = &module.opcodes[6] {
        assert_eq!(s.to_string(), "x");
    } else {
        assert!(false);
    }
    assert!(matches!(module.opcodes[7], Opcode::Ret()));
    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_func_call() {
    let mut tokenizer = Tokenizer::new("fn main {let x = main();main();}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 12);

    assert!(matches!(module.opcodes[0], Opcode::BeginFnArgs()));
    assert!(matches!(module.opcodes[1], Opcode::EndFnArgs()));
    if let Opcode::LoadVar(s) = &module.opcodes[2] {
        assert_eq!(s.to_string(), "main");
    } else {
        assert!(false);
    }
    assert!(matches!(module.opcodes[3], Opcode::Call()));
    assert!(matches!(module.opcodes[4], Opcode::PushRet()));
    if let Opcode::StoreVar(s) = &module.opcodes[5] {
        assert_eq!(s.to_string(), "x");
    } else {
        assert!(false);
    }

    assert!(matches!(module.opcodes[6], Opcode::BeginFnArgs()));
    assert!(matches!(module.opcodes[7], Opcode::EndFnArgs()));
    if let Opcode::LoadVar(s) = &module.opcodes[8] {
        assert_eq!(s.to_string(), "main");
    } else {
        assert!(false);
    }
    assert!(matches!(module.opcodes[9], Opcode::Call()));

    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_load_string() {
    let mut tokenizer = Tokenizer::new("fn main {return \"Hello\"; }".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 2);

    if let Opcode::LoadStr(s) = &module.opcodes[0] {
        assert_eq!(s.to_string(), "Hello");
    } else {
        assert!(false);
    }
    assert!(matches!(module.opcodes[1], Opcode::Ret()));

    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].addr, 0);
    assert_eq!(module.functions[0].name, "main");
    assert_eq!(module.functions[0].args_count, 0);
}

#[test]
fn compiler_test_main_function_shall_not_accept_args() {
    let mut tokenizer = Tokenizer::new("fn main x {}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "main function should not accept any arguments");
    } else {
        assert!(false);
    }
}

#[test]
fn compiler_test_implicit_null() {
    let mut tokenizer = Tokenizer::new("fn main {}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }
    assert_eq!(compiler.warnings.len(), 1);
    assert_eq!(
        compiler.warnings[0],
        "function main implicitly returns null"
    );
}
