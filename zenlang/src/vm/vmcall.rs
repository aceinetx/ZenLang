use crate::value::*;
use crate::vm::*;
use alloc::format;

impl<'a> VM<'a> {
    /// Performs a vmcall
    ///
    /// ### VMCall indexes
    /// - 1: print
    /// - 2: println
    /// - 3: get_string
    pub fn vmcall(&mut self, index: u8) {
        match index {
            1 => {
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.print(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            2 => {
                if let Some(platform) = &self.platform {
                    if let Some(value) = self.stack.pop() {
                        platform.println(format!("{}", value));
                        return;
                    }
                    self.error = "vmcall: no value on stack".into();
                }
            }
            3 => {
                if let Some(platform) = &self.platform {
                    let string = platform.get_string();
                    let value = Value::String(string.into());
                    self.stack.push(value);
                }
            }
            _ => {
                self.error = format!("vmcall: invalid vmcall index {}", index);
            }
        }
    }
}
