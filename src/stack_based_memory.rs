use crate::memory::MemoryVal;
use alloc::prelude::*;
use alloc::rc::Rc;
use byteorder::{ByteOrder, LittleEndian};
use core::cell::RefCell;
use core::convert;

const DEFAULT_MAX_SIZE: usize = 65_536;

#[derive(Debug, Clone)]
pub enum StackValType {
    Utf8,
    Utf16,
    Raw,
}

#[derive(Debug, Clone)]
pub struct StackVal {
    pub data: Vec<u8>,
    pub val_type: StackValType,
}

impl<T: MemoryVal> convert::From<T> for StackVal {
    fn from(val: T) -> StackVal {
        StackVal {
            data: val.vec(),
            val_type: T::val_type(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackBasedMemory {
    size: Rc<RefCell<usize>>,
    max_size: usize,
    values: Rc<RefCell<Vec<StackVal>>>,
}

impl StackBasedMemory {
    pub fn default() -> StackBasedMemory {
        StackBasedMemory {
            size: Rc::new(RefCell::new(0)),
            max_size: DEFAULT_MAX_SIZE,
            values: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, val: StackVal) -> Result<(), String> {
        let val_size = val.data.len();
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
                .replace_with(|&mut old_size| old_size - val.data.len());
            Some(val)
        } else {
            None
        }
    }
}
