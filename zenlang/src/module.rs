use crate::opcode::Opcode;
use ::serde::{Deserialize, Serialize};
use alloc::string::String;
use alloc::vec::Vec;
use bincode;
use bincode::config::Configuration;
use bincode::error::DecodeError;
use bincode::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug)]
pub struct ModuleFunction {
    pub name: String,
    pub addr: u32,
    pub args_count: u64,
}

impl ModuleFunction {
    pub fn new(name: String, addr: u32, args_count: u64) -> ModuleFunction {
        return ModuleFunction {
            name: name,
            addr: addr,
            args_count: args_count,
        };
    }
}

#[derive(Encode, Decode, Debug)]
pub struct Module {
    pub opcodes: Vec<Opcode>,
    pub functions: Vec<ModuleFunction>,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            opcodes: Vec::new(),
            functions: Vec::new(),
        };
    }

    pub fn compile(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let cfg = bincode::config::standard();
        let bytes = bincode::encode_to_vec(self, cfg);
        return bytes;
    }

    pub fn load(&mut self, bytes: Vec<u8>) -> Result<(), DecodeError> {
        let cfg = bincode::config::standard();
        match bincode::decode_from_slice::<Module, Configuration>(&bytes, cfg) {
            Err(e) => {
                return Err(e);
            }
            Ok(new) => {
                *self = new.0;
                return Ok(());
            }
        }
    }

    pub fn get_opcode(&self, addr: u32) -> &Opcode {
        return &self.opcodes[addr as usize];
    }
}

impl Default for Module {
    fn default() -> Self {
        return Self::new();
    }
}
