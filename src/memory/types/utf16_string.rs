use super::{Array, Raw, TypedArray, Utf8String};
use crate::memory::{
    DynLittleEndianConvert, MemoryDescriptor, MemoryVal, StorageVal, StorageValType,
};
use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasmi::MemoryRef;

#[derive(Debug, Clone)]
pub struct Utf16String(Vec<u8>, MemoryDescriptor);

impl Utf16String {
    pub fn default(val: Vec<u8>) -> Utf16String {
        let descriptor = MemoryDescriptor {
            length: (val.len() / 2) as u32,
            elem_size: 2,
        };
        Utf16String::new(val, descriptor)
    }

    pub fn string(&self) -> Result<String, RuntimeError> {
        let len = self.bytes().len() / 2;
        let mut dest = Vec::<u16>::with_capacity(len);
        dest.resize(len, 0);
        LittleEndian::read_u16_into(self.bytes(), &mut dest);
        String::from_utf16(&dest).map_err(|_| RuntimeError::new("Failed to decode from utf16"))
    }
}

impl DynLittleEndianConvert for Utf16String {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError> {
        let size = memory.get(offset, 4).map(|s| LittleEndian::read_u32(&s))?;
        let descriptor = MemoryDescriptor {
            length: size,
            elem_size: 2,
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

impl MemoryVal for Utf16String {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> Utf16String {
        Utf16String(data, descriptor)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Utf16
    }

    fn written_size(&self) -> usize {
        self.bytes().len() + 4
    }
}

impl TryFrom<Raw> for Utf16String {
    type Error = RuntimeError;

    fn try_from(val: Raw) -> Result<Utf16String, Self::Error> {
        match val.elem_size() {
            1 => Ok(Utf16String::from(Utf8String::try_from(val)?)),
            2 => {
                if val.size() % 2 != 0 {
                    Err(RuntimeError::new(
                        "Array does not have an even number of bytes",
                    ))
                } else {
                    Ok(Utf16String::default(val.vec()))
                }
            }
            _ => Err(RuntimeError::new("Invalid element size")),
        }
    }
}

impl From<Utf8String> for Utf16String {
    fn from(val: Utf8String) -> Utf16String {
        let size = val.bytes().len();
        let mut padding = Vec::<u8>::with_capacity(size);
        padding.resize(size, 0);
        let new_bytes = val
            .bytes()
            .iter()
            .zip(&padding)
            .map(|(v, p)| [*v, *p].to_vec())
            .flatten()
            .collect();
        Utf16String::default(new_bytes)
    }
}

impl TryFrom<Array> for Utf16String {
    type Error = RuntimeError;

    fn try_from(val: Array) -> Result<Utf16String, Self::Error> {
        Utf16String::try_from(Raw::from(val))
    }
}

impl TryFrom<TypedArray> for Utf16String {
    type Error = RuntimeError;

    fn try_from(val: TypedArray) -> Result<Utf16String, Self::Error> {
        Utf16String::try_from(Raw::from(val))
    }
}

impl TryFrom<StorageVal> for Utf16String {
    type Error = RuntimeError;

    fn try_from(val: StorageVal) -> Result<Utf16String, Self::Error> {
        match val.val_type {
            StorageValType::Utf16 => Ok(Utf16String::default(val.data)),
            StorageValType::Utf8 => Ok(Utf8String::try_from(val)?.into()),
            StorageValType::Raw => Utf16String::try_from(Raw::from(val)),
            StorageValType::Array => Utf16String::try_from(Array::try_from(val)?),
            StorageValType::TypedArray => Utf16String::try_from(TypedArray::try_from(val)?),
        }
    }
}
