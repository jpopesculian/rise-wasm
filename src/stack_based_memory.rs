use alloc::prelude::*;
use alloc::rc::Rc;
use byteorder::{ByteOrder, LittleEndian};
use core::cell::RefCell;
use wasmi::memory_units::Pages;
use wasmi::{MemoryInstance, MemoryRef};

const DEFAULT_MAX_SIZE: usize = 65_536;

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

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        let size = self.size();
        match self.encoding {
            StackValEncoding::Utf16 => size / 2,
            _ => size,
        }
    }

    pub fn to_utf8(mut self) -> Result<StackVal, String> {
        match self.encoding {
            // TODO remove padding if possible
            StackValEncoding::Utf16 => Err("Utf16 cannot be converted to utf8".to_string()),
            _ => {
                self.encoding = StackValEncoding::Utf8;
                Ok(self)
            }
        }
    }

    pub fn to_utf16(mut self) -> Result<StackVal, String> {
        match self.encoding {
            StackValEncoding::Utf8 => {
                let size = self.size();
                let mut padding = Vec::<u8>::with_capacity(size);
                padding.resize(size, 0);
                self.data = self
                    .data
                    .iter()
                    .zip(&padding)
                    .map(|(v, p)| [*v, *p].to_vec())
                    .flatten()
                    .collect();
                self.encoding = StackValEncoding::Utf16;
                Ok(self)
            }
            StackValEncoding::Raw => {
                if self.size() % 2 != 0 {
                    Err("Array does not have an even number of bytes".to_string())
                } else {
                    self.encoding = StackValEncoding::Utf16;
                    Ok(self)
                }
            }
            StackValEncoding::Utf16 => Ok(self),
        }
    }

    pub fn to_raw(mut self) -> StackVal {
        self.encoding = StackValEncoding::Raw;
        self
    }

    pub fn string(self) -> Result<String, String> {
        match self.encoding {
            StackValEncoding::Utf16 => {
                let len = self.size() / 2;
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
    size: Rc<RefCell<usize>>,
    max_size: usize,
    values: Rc<RefCell<Vec<StackVal>>>,
}

impl StackBasedMemory {
    pub fn default() -> StackBasedMemory {
        StackBasedMemory {
            memory: StackBasedMemory::build_memory(),
            size: Rc::new(RefCell::new(0)),
            max_size: DEFAULT_MAX_SIZE,
            values: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn build_memory() -> MemoryRef {
        MemoryInstance::alloc(Pages(1), Some(Pages(1))).expect("Memory could not be initialized")
    }

    pub fn memory(&self) -> MemoryRef {
        self.memory.clone()
    }

    pub fn push(&self, val: StackVal) -> Result<(), String> {
        let val_size = val.size();
        if *self.size.borrow() + val_size > self.max_size {
            return Err("Stack overflow!".to_string());
        }
        self.size.replace_with(|&mut old_size| old_size + val_size);
        self.values.borrow_mut().push(val);
        Ok(())
    }

    pub fn pop(&self) -> Option<StackVal> {
        if let Some(val) = self.values.borrow_mut().pop() {
            self.size
                .replace_with(|&mut old_size| old_size - val.size());
            Some(val)
        } else {
            None
        }
    }
}
