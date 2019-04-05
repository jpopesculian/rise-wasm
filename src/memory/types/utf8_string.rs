use super::{Array, Raw, TypedArray, Utf16String};
use crate::memory::{
    DynLittleEndianConvert, MemoryDescriptor, MemoryVal, StorageVal, StorageValType,
};
use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasmi::MemoryRef;

#[derive(Debug, Clone)]
pub struct Utf8String(Vec<u8>, MemoryDescriptor);

impl Utf8String {
    pub fn default(val: Vec<u8>) -> Utf8String {
        let descriptor = MemoryDescriptor {
            length: val.len() as u32,
            elem_size: 1,
        };
        Utf8String::new(val, descriptor)
    }

    pub fn string(&self) -> Result<String, RuntimeError> {
        String::from_utf8(self.bytes().to_vec())
            .map_err(|_| RuntimeError::new("Failed to decode from utf8"))
    }
}

impl DynLittleEndianConvert for Utf8String {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError> {
        let size = memory.get(offset, 4).map(|s| LittleEndian::read_u32(&s))?;
        let descriptor = MemoryDescriptor {
            length: size,
            elem_size: 1,
        };
        Ok((offset + 4, descriptor))
    }

    fn into_little_endian(self, buffer: &mut [u8], _offset: u32) {
        let mut len_descriptor = [0; 4];
        LittleEndian::write_u32(&mut len_descriptor, self.len());
        let mut result = len_descriptor.to_vec();
        result.append(&mut self.vec());
        buffer.copy_from_slice(&result);
    }
}

impl MemoryVal for Utf8String {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> Utf8String {
        Utf8String(data, descriptor)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Utf8
    }

    fn written_size(&self) -> usize {
        self.bytes().len() + 4
    }
}

impl TryFrom<Raw> for Utf8String {
    type Error = RuntimeError;

    fn try_from(val: Raw) -> Result<Utf8String, Self::Error> {
        match val.elem_size() {
            1 => Ok(Utf8String::default(val.vec())),
            2 => Utf8String::try_from(Utf16String::try_from(val)?),
            _ => Err(RuntimeError::new("Invalid element size")),
        }
    }
}

impl TryFrom<Array> for Utf8String {
    type Error = RuntimeError;

    fn try_from(val: Array) -> Result<Utf8String, Self::Error> {
        Utf8String::try_from(Raw::from(val))
    }
}

impl TryFrom<TypedArray> for Utf8String {
    type Error = RuntimeError;

    fn try_from(val: TypedArray) -> Result<Utf8String, Self::Error> {
        Utf8String::try_from(Raw::from(val))
    }
}

impl TryFrom<Utf16String> for Utf8String {
    type Error = RuntimeError;
    fn try_from(val: Utf16String) -> Result<Utf8String, Self::Error> {
        let mut result = Vec::<u8>::new();
        let bytes = val.bytes();
        for (i, byte) in bytes.iter().enumerate().step_by(2) {
            let next: u8 = *bytes
                .get(i + 1)
                .ok_or(RuntimeError::new("String not divisible by two"))?;
            if next != 0 {
                return Err(RuntimeError::new("String not valid utf8"));
            }
            result.push(*byte);
        }
        Ok(Utf8String::default(result))
    }
}

impl TryFrom<StorageVal> for Utf8String {
    type Error = RuntimeError;

    fn try_from(val: StorageVal) -> Result<Utf8String, Self::Error> {
        match val.val_type {
            StorageValType::Utf16 => Utf8String::try_from(Utf16String::try_from(val)?),
            StorageValType::Utf8 => Ok(Utf8String::default(val.data)),
            StorageValType::Raw => Utf8String::try_from(Raw::from(val)),
            StorageValType::Array => Utf8String::try_from(Array::try_from(val)?),
            StorageValType::TypedArray => Utf8String::try_from(TypedArray::try_from(val)?),
        }
    }
}
