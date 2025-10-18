use crate::value::*;
use crate::vm::VM;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::*;
use alloc::string::*;
use alloc::vec::*;
use core::cell::*;

impl VM {
    pub fn op_cdfse(&mut self, names: &Vec<String>) {
        let mut dict = BTreeMap::<String, Value>::new();
        for i in 0..names.len() {
            if let Some(stack_value) = self.stack.pop() {
                // todo: do smth w ts clone
                //items.insert(0, (names[names.len() - i - 1].clone(), stack_value));
                dict.insert(names[names.len() - i - 1].clone(), stack_value);
            } else {
                self.error = format!("cdfse failed: no more values on stack");
                return;
            }
        }

        let obj = Rc::new(RefCell::new(Object::Dictionary(dict)));
        let v = Value::Object(obj);
        self.stack.push(v);
    }
}
