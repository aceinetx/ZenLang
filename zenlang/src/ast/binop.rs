use crate::ast::node::StatementExpression;
use crate::compiler::Compiler;
use crate::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::String;

pub enum AstBinopOp {
    PLUS,
    MINUS,
    MUL,
    DIV,
    EQ,
    NEQ,
    LT,
    GT,
    LE,
    GE,
    BITSHR,
    BITSHL,
    BITAND,
    BITOR,
}

pub struct AstBinop {
    pub left: Box<dyn Compile>,
    pub right: Box<dyn Compile>,
    pub op: AstBinopOp,
    do_push: bool,
}

impl AstBinop {
    pub fn new(left: Box<dyn Compile>, op: AstBinopOp, right: Box<dyn Compile>) -> Self {
        return Self {
            left: left,
            right: right,
            op: op,
            do_push: true,
        };
    }
}

impl Compile for AstBinop {
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        self.left.compile(compiler)?;
        self.right.compile(compiler)?;

        let opcode;

        match self.op {
            AstBinopOp::PLUS => {
                opcode = Opcode::Add();
            }
            AstBinopOp::MINUS => {
                opcode = Opcode::Sub();
            }
            AstBinopOp::MUL => {
                opcode = Opcode::Mul();
            }
            AstBinopOp::DIV => {
                opcode = Opcode::Div();
            }
            AstBinopOp::EQ => {
                opcode = Opcode::Eq();
            }
            AstBinopOp::NEQ => {
                opcode = Opcode::Neq();
            }
            AstBinopOp::LT => {
                opcode = Opcode::Lt();
            }
            AstBinopOp::GT => {
                opcode = Opcode::Gt();
            }
            AstBinopOp::LE => {
                opcode = Opcode::Le();
            }
            AstBinopOp::GE => {
                opcode = Opcode::Ge();
            }
            AstBinopOp::BITSHR => opcode = Opcode::Bshr(),
            AstBinopOp::BITSHL => opcode = Opcode::Bshl(),
            AstBinopOp::BITAND => opcode = Opcode::Band(),
            AstBinopOp::BITOR => opcode = Opcode::Bor(),
        }

        let module = compiler.get_module();
        module.opcodes.push(opcode);
        if !self.do_push {
            module.opcodes.push(Opcode::Pop());
        }

        Ok(())
    }
}

impl StatementExpression for AstBinop {
    fn disable_push(&mut self) {
        self.do_push = false;
    }
}
