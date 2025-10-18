use crate::value::*;
use crate::vm::VM;
use alloc::format;
use alloc::rc::*;
use alloc::vec::*;
use core::cell::*;

impl VM {
    pub fn op_cafse(&mut self, items: u64) {
        let mut vec = Vec::<Value>::new();
        for _ in 0..items {
            if let Some(stack_value) = self.stack.pop() {
                vec.insert(0, stack_value);
            } else {
                self.error = format!("cafse failed: no more values on stack");
                return;
            }
        }
        let obj = Rc::new(RefCell::new(Object::Array(vec)));

        let v = Value::Object(obj);

        self.stack.push(v);
    }
}
