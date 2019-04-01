use crate::memory::{MemoryDescriptor, MemoryVal};
use alloc::prelude::*;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::convert;
use hashbrown::HashMap;

const DEFAULT_MAX_SIZE: usize = 65_536;

#[derive(Debug, Clone)]
pub enum StorageValType {
    Utf8,
    Utf16,
    Raw,
    Array,
    TypedArray,
}

#[derive(Debug, Clone)]
pub struct StorageVal {
    pub data: Vec<u8>,
    pub val_type: StorageValType,
    pub descriptor: MemoryDescriptor,
}

impl StorageVal {
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn vec(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }

    pub fn size(&self) -> u32 {
        self.bytes().len() as u32
    }
}

impl<T: MemoryVal> convert::From<T> for StorageVal {
    fn from(val: T) -> StorageVal {
        StorageVal {
            data: val.vec(),
            val_type: T::val_type(),
            descriptor: val.descriptor().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableStorage {
    size: Rc<RefCell<usize>>,
    max_size: usize,
    values: Rc<RefCell<HashMap<u32, StorageVal>>>,
}

impl TableStorage {
    pub fn default() -> TableStorage {
        TableStorage {
            size: Rc::new(RefCell::new(0)),
            max_size: DEFAULT_MAX_SIZE,
            values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn insert(&self, index: u32, val: StorageVal) -> Result<(), String> {
        let val_size = val.data.len();
        if *self.size.borrow() + val_size > self.max_size {
            return Err("Table overflow!".to_string());
        }
        self.size.replace_with(|&mut old_size| old_size + val_size);
        self.values.borrow_mut().insert(index, val);
        Ok(())
    }

    pub fn get(&self, index: &u32) -> Option<StorageVal> {
        if let Some(val) = self.values.borrow_mut().get(index) {
            self.size
                .replace_with(|&mut old_size| old_size - val.data.len());
            Some(val.clone())
        } else {
            None
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct StackStorage {
//     size: Rc<RefCell<usize>>,
//     max_size: usize,
//     values: Rc<RefCell<Vec<StorageVal>>>,
// }

// impl StackStorage {
//     pub fn default() -> StackStorage {
//         StackStorage {
//             size: Rc::new(RefCell::new(0)),
//             max_size: DEFAULT_MAX_SIZE,
//             values: Rc::new(RefCell::new(Vec::new())),
//         }
//     }

//     pub fn push(&self, val: StorageVal) -> Result<(), String> {
//         let val_size = val.data.len();
//         if *self.size.borrow() + val_size > self.max_size {
//             return Err("Stack overflow!".to_string());
//         }
//         self.size.replace_with(|&mut old_size| old_size + val_size);
//         self.values.borrow_mut().push(val);
//         Ok(())
//     }

//     pub fn pop(&self) -> Option<StorageVal> {
//         if let Some(val) = self.values.borrow_mut().pop() {
//             self.size
//                 .replace_with(|&mut old_size| old_size - val.data.len());
//             Some(val)
//         } else {
//             None
//         }
//     }
// }
