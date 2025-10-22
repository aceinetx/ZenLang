use crate::opcode::*;
use crate::strong_u64::U64BitsControl;
use crate::value::Value;
use crate::vm::*;
use alloc::format;

impl VM {
    pub(crate) fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Call() => {
                self.op_call();
            }
            Opcode::Vmcall(index) => {
                self.op_vmcall(*index);
            }
            Opcode::Dynvmcall() => {
                self.op_dynvmcall();
            }
            Opcode::LoadConstant(value) => {
                self.op_load_constant(*value);
            }
            Opcode::LoadNull() => {
                self.op_load_null();
            }
            Opcode::LoadBool(flag) => {
                self.op_load_bool(*flag);
            }
            Opcode::LoadStr(value) => {
                self.op_load_str(value);
            }
            Opcode::LoadVar(name) => {
                self.op_load_var(name);
            }
            Opcode::StoreVar(name) => {
                self.op_store_var(name);
            }
            Opcode::PushRet() => {
                self.op_push_ret();
            }
            Opcode::Cafse(items) => {
                self.op_cafse(*items);
            }
            Opcode::Iafs() => {
                self.op_iafs();
            }
            Opcode::Cdfse(names) => {
                self.op_cdfse(names);
            }
            Opcode::Aiafs() => {
                self.op_aiafs();
            }
            Opcode::BeginFnArgs() => {
                self.op_begin_fn_args();
            }
            Opcode::EndFnArgs() => {
                self.op_end_fn_args();
            }
            Opcode::Pop() => {
                if self.stack.is_empty() {
                    self.error = format!("pop failed: no value in stack");
                } else {
                    self.stack.pop();
                }
            }
            Opcode::BranchTrue(addr) => {
                self.op_branch_true(*addr);
            }
            Opcode::BranchNonNull(addr) => {
                self.op_branch_nonnull(*addr);
            }
            Opcode::Branch(addr) => {
                self.op_branch(*addr);
            }
            Opcode::Add() => {
                self.op_add();
            }
            Opcode::Sub() => {
                self.op_sub();
            }
            Opcode::Mul() => {
                self.op_mul();
            }
            Opcode::Div() => {
                self.op_div();
            }
            Opcode::Eq() => {
                self.op_eq();
            }
            Opcode::Neq() => {
                self.op_neq();
            }
            Opcode::Lt() => {
                self.op_lt();
            }
            Opcode::Gt() => {
                self.op_gt();
            }
            Opcode::Le() => {
                self.op_le();
            }
            Opcode::Ge() => {
                self.op_ge();
            }
            Opcode::Bshr() => {
                self.op_bshr();
            }
            Opcode::Bshl() => {
                self.op_bshl();
            }
            Opcode::Band() => {
                self.op_band();
            }
            Opcode::Bor() => {
                self.op_bor();
            }
            Opcode::Ret() => {
                self.op_ret();
            }
            Opcode::Closure(skip, args) => {
                let value = Value::FunctionRefEnv(
                    self.pc.get_low() as u64,
                    *args,
                    self.environs.last().unwrap().clone(),
                );

                self.pc.add_low(*skip);

                self.stack.push(value);
                //panic!();
            }
        }
    }
}
