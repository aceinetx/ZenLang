use crate::{ast::node::Compile, opcode::Opcode};
use alloc::vec::*;

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
    pub left: Option<alloc::boxed::Box<dyn Compile>>,
    pub right: Option<alloc::boxed::Box<dyn Compile>>,
    pub op: AstBinopOp,
    do_push: bool,
}

impl AstBinop {
    pub fn new() -> Self {
        return Self {
            left: None,
            right: None,
            op: AstBinopOp::PLUS,
            do_push: true,
        };
    }
}

impl Compile for AstBinop {
    fn disable_push(&mut self) {
        self.do_push = false;
    }

    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if let Some(left) = &mut self.left {
            if let Err(e) = left.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("left is None".into());
        }
        if let Some(right) = &mut self.right {
            if let Err(e) = right.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("right is None".into());
        }

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
