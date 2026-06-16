use crate::value::*;
use crate::vm::*;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::*;
use alloc::string::*;
use alloc::vec::*;
use core::cell::*;

impl VM {
    pub fn op_cdfse(&mut self, names: &Vec<String>) -> Result<(), VMError> {
        let mut dict = BTreeMap::<String, Value>::new();
        for i in 0..names.len() {
            let stack_value = match self.stack.pop() {
                Some(value) => value,
                None => {
                    return Err("cdfse failed: no more values on stack".into());
                }
            };

            dict.insert(names[names.len() - i - 1].clone(), stack_value);
        }

        let obj = Rc::new(RefCell::new(Object::Dictionary(dict)));
        let v = Value::Object(obj);
        self.stack.push(v);

        return Ok(());
    }
}
