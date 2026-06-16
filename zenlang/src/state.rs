use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

use crate::vm::GlobalState;
use crate::vm::VM;
use crate::vm::VMError;

pub struct State {
    pub vms: Vec<VM>,
    pub global_state: Rc<RefCell<GlobalState>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn create_vm(&mut self) {
        self.vms.push(VM::new(self.global_state.clone()));
    }

    pub fn is_all_halted(&self) -> bool {
        for vm in self.vms.iter() {
            if !vm.halted {
                return false;
            }
        }
        return false;
    }

    pub fn run_until_halt(&mut self) -> Result<(), VMError> {
        while !self.is_all_halted() {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VMError> {
        for vm in self.vms.iter_mut() {
            vm.step()?;
        }
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
