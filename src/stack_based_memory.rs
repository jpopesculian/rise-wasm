use alloc::prelude::*;
use alloc::rc::Rc;
use byteorder::{ByteOrder, LittleEndian};
use core::cell::RefCell;
use core::fmt::Error;
use wasmi::memory_units::Pages;
use wasmi::{MemoryInstance, MemoryRef};

// @TODO: implement max size

#[derive(Debug, Clone)]
pub enum StackValEncoding {
    Utf8,
    Utf16,
    Raw,
}

#[derive(Debug, Clone)]
pub struct StackVal {
    data: Vec<u8>,
    encoding: StackValEncoding,
}

impl StackVal {
    pub fn utf8(data: Vec<u8>) -> StackVal {
        StackVal {
            encoding: StackValEncoding::Utf8,
            ..StackVal::default(data)
        }
    }

    pub fn utf16(data: Vec<u8>) -> StackVal {
        StackVal {
            encoding: StackValEncoding::Utf16,
            ..StackVal::default(data)
        }
    }

    pub fn default(data: Vec<u8>) -> StackVal {
        StackVal {
            data,
            encoding: StackValEncoding::Raw,
        }
    }

    pub fn data(self) -> Vec<u8> {
        self.data
    }

    pub fn string(self) -> Result<String, String> {
        match self.encoding {
            StackValEncoding::Utf16 => {
                let len = self.data.len() / 2;
                let mut dest = Vec::<u16>::with_capacity(len);
                dest.resize(len, 0);
                LittleEndian::read_u16_into(&self.data, &mut dest);
                String::from_utf16(&dest).map_err(|_| "Failed to decode from utf16".to_string())
            }
            _ => String::from_utf8(self.data).map_err(|_| "Failed to decode from utf8".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackBasedMemory {
    memory: MemoryRef,
    values: Rc<RefCell<Vec<StackVal>>>,
}

impl StackBasedMemory {
    pub fn new() -> StackBasedMemory {
        StackBasedMemory {
            memory: StackBasedMemory::build_memory(),
            values: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn build_memory() -> MemoryRef {
        MemoryInstance::alloc(Pages(1), Some(Pages(1))).expect("Memory could not be initialized")
    }

    pub fn memory(&self) -> MemoryRef {
        self.memory.clone()
    }

    pub fn push(&self, val: StackVal) -> Result<(), Error> {
        self.values.borrow_mut().push(val);
        Ok(())
    }

    pub fn pop(&self) -> Option<StackVal> {
        self.values.borrow_mut().pop()
    }
}
