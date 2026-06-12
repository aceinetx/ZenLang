use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgramCounter {
    pub module: usize,
    pub inst: usize,
}

impl ProgramCounter {
    pub fn new() -> Self {
        return Self { module: 0, inst: 0 };
    }
}

impl Default for ProgramCounter {
    fn default() -> Self {
        return Self::new();
    }
}

impl Display for ProgramCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({};{})", self.module, self.inst)
    }
}
