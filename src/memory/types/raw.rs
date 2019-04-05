use super::{Array, TypedArray, Utf16String, Utf8String};
use crate::memory::{MemoryDescriptor, MemoryVal, StorageVal, StorageValType};
use alloc::prelude::*;
use core::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Raw(Vec<u8>, MemoryDescriptor);

impl Raw {
    pub fn default(data: Vec<u8>) -> Raw {
        let descriptor = MemoryDescriptor {
            length: data.len() as u32,
            elem_size: 1,
        };
        Raw::new(data, descriptor)
    }
}

impl MemoryVal for Raw {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> Raw {
        Raw(data, descriptor)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Raw
    }

    fn written_size(&self) -> usize {
        self.bytes().len()
    }
}

impl From<Utf16String> for Raw {
    fn from(val: Utf16String) -> Raw {
        Raw::new(val.vec(), val.descriptor().clone())
    }
}

impl From<Utf8String> for Raw {
    fn from(val: Utf8String) -> Raw {
        Raw::new(val.vec(), val.descriptor().clone())
    }
}

impl From<Array> for Raw {
    fn from(val: Array) -> Raw {
        Raw::new(val.vec(), val.descriptor().clone())
    }
}

impl From<TypedArray> for Raw {
    fn from(val: TypedArray) -> Raw {
        Raw::new(val.vec(), val.descriptor().clone())
    }
}

impl From<StorageVal> for Raw {
    fn from(val: StorageVal) -> Raw {
        match val.val_type {
            StorageValType::Utf16 => Utf16String::try_from(val).unwrap().into(),
            StorageValType::Utf8 => Utf8String::try_from(val).unwrap().into(),
            StorageValType::Raw => Raw::new(val.data, val.descriptor),
            StorageValType::Array => Array::try_from(val).unwrap().into(),
            StorageValType::TypedArray => TypedArray::try_from(val).unwrap().into(),
        }
    }
}
