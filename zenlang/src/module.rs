//! Module
//!
//! Contains ZenLang module structs
use crate::opcode::Opcode;
use ::serde::{Deserialize, Serialize};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use bincode;
use bincode::config::Configuration;
use bincode::error::DecodeError;
use bincode::*;

/// ModuleFunction
///
/// Contains information about a module function
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct ModuleFunction {
    /// Function name
    pub name: String,
    /// Function address relative to the module offset
    pub addr: u32,
    /// Argument count
    pub args_count: u64,
    /// Is a constructor function
    pub ctor: bool,
}

impl ModuleFunction {
    pub fn new(name: String, addr: u32, args_count: u64, ctor: bool) -> ModuleFunction {
        return ModuleFunction {
            name: name,
            addr: addr,
            args_count: args_count,
            ctor: ctor,
        };
    }
}

/// Module
///
/// Contains module information (code)
#[derive(Encode, Decode, Debug, Clone)]
pub struct Module {
    /// Module dependencies
    pub dependencies: Vec<String>,
    /// Module name
    pub name: String,
    /// Function informations
    pub functions: Vec<ModuleFunction>,
    /// Opcodes of the module (entire code)
    pub opcodes: Vec<Opcode>,
    /// Global variables
    pub globals: Vec<String>,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            opcodes: Vec::new(),
            functions: Vec::new(),
            dependencies: Vec::new(),
            name: String::new(),
            globals: Vec::new(),
        };
    }

    /// Compiles the module into bytes vector (Serializes)
    pub fn compile(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let cfg = bincode::config::standard();
        let bytes = bincode::encode_to_vec(self, cfg);
        return bytes;
    }

    /// Load the module from bytes vector (Deserializes)
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

    /// Get an opcode at a certain address
    pub fn get_opcode(&self, addr: u32) -> &Opcode {
        return &self.opcodes[addr as usize];
    }

    pub fn format_debug_opcodes(&self) -> String {
        let mut s = String::new();

        for (index, opcode) in self.opcodes.iter().enumerate() {
            s.push_str(format!("{:>width$} {:?}\n", index, opcode, width = 5).as_str());
        }

        return s;
    }
}

impl Default for Module {
    fn default() -> Self {
        return Self::new();
    }
}
