use crate::strong_u64::U64BitsControl;
use crate::value::Value;
use crate::vm::VM;

impl VM {
    pub fn op_closure(&mut self, skip: u32, args: u64) {
        let value = Value::FunctionRefEnv(
            self.pc.get_low() as u64,
            args,
            self.environs.last().unwrap().clone(),
        );

        self.pc.add_low(skip);

        self.stack.push(value);
    }
}
