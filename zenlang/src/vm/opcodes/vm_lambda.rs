use core::cell::RefCell;

use alloc::rc::Rc;

use crate::{
    value::Value,
    vm::{ProgramCounter, VM},
};

impl VM {
    pub fn op_lambda(&mut self, pc: usize, args: usize) {
        let scope = self.scopes.last().unwrap().clone();
        let scope = Rc::new(RefCell::new(scope));
        let lambda = Value::Lambda(ProgramCounter::with(self.pc.module, pc), scope, args);
        self.stack.push(lambda);
    }
}
