use zenlang::compiler::*;
use zenlang::opcode::*;
use zenlang::parser::*;
use zenlang::tokenizer::*;

#[test]
fn compiler_test_func_naked() {
    let mut tokenizer = Tokenizer::new("fn #[naked] func n {return null;}".into());
    let mut parser = Parser::new(&mut tokenizer);
    let mut compiler = Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        assert_eq!(e, "");
    }

    let module = compiler.get_module();
    assert_eq!(module.opcodes.len(), 2);
    assert_eq!(module.functions.len(), 1);
    assert!(matches!(module.opcodes[0], Opcode::Loadcnu()));
    assert!(matches!(module.opcodes[1], Opcode::Ret()));
}
