use super::{Raw, TypedArray, Utf16String, Utf8String};
use crate::memory::{
    DynLittleEndianConvert, MemoryDescriptor, MemoryVal, StorageVal, StorageValType,
};
use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasmi::MemoryRef;

#[derive(Debug, Clone)]
pub struct Array(Vec<u8>, MemoryDescriptor);

// impl Array {
//     pub fn with_elem_size(data: Vec<u8>, elem_size: u32) -> Result<Array, String> {
//         let length = data.len() as u32;
//         if length % elem_size != 0 {
//             Err("Data cannot be split into element size".to_string())
//         } else {
//             Ok(Array::new(data, MemoryDescriptor { length, elem_size }))
//         }
//     }

//     pub fn default(data: Vec<u8>) -> Array {
//         let length = data.len() as u32;
//         Array::new(
//             data,
//             MemoryDescriptor {
//                 length,
//                 elem_size: 1,
//             },
//         )
//     }
// }

impl MemoryVal for Array {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> Array {
        Array(data, descriptor)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Array
    }

    fn written_size(&self) -> usize {
        self.bytes().len() + 16
    }
}

impl DynLittleEndianConvert for Array {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError> {
        let size_offset = memory.get(offset, 4).map(|s| LittleEndian::read_u32(&s))?;
        let size = memory
            .get(size_offset, 4)
            .map(|s| LittleEndian::read_u32(&s))?;
        let length = memory
            .get(offset + 4, 4)
            .map(|s| LittleEndian::read_u32(&s))?;
        let descriptor = MemoryDescriptor {
            length,
            elem_size: size / length,
        };
        Ok((size_offset + 8, descriptor))
    }

    fn into_little_endian(self, buffer: &mut [u8], offset: u32) {
        let mut offset_descriptor = [0; 4];
        let mut len_descriptor = [0; 4];
        let mut size_descriptor = [0; 4];
        let alignment = [0; 4];
        LittleEndian::write_u32(&mut size_descriptor, self.size());
        LittleEndian::write_u32(&mut len_descriptor, self.len());
        LittleEndian::write_u32(&mut offset_descriptor, offset + 8);

        let mut result = offset_descriptor.to_vec();
        result.append(&mut len_descriptor.to_vec());
        result.append(&mut size_descriptor.to_vec());
        result.append(&mut alignment.to_vec());
        result.append(&mut self.vec());
        buffer.copy_from_slice(&result);
    }
}

impl TryFrom<StorageVal> for Array {
    type Error = RuntimeError;

    fn try_from(val: StorageVal) -> Result<Array, Self::Error> {
        match val.val_type {
            StorageValType::Utf16 => Ok(Array::from(Utf16String::try_from(val)?)),
            StorageValType::Utf8 => Ok(Array::from(Utf8String::try_from(val)?)),
            StorageValType::Raw => Ok(Array::from(Raw::from(val))),
            StorageValType::Array => Ok(Array::new(val.data, val.descriptor)),
            StorageValType::TypedArray => Ok(Array::from(TypedArray::try_from(val)?)),
        }
    }
}

impl From<Raw> for Array {
    fn from(val: Raw) -> Array {
        Array::new(val.vec(), val.descriptor().clone())
    }
}

impl From<Utf8String> for Array {
    fn from(val: Utf8String) -> Array {
        Array::new(val.vec(), val.descriptor().clone())
    }
}

impl From<Utf16String> for Array {
    fn from(val: Utf16String) -> Array {
        Array::new(val.vec(), val.descriptor().clone())
    }
}

impl From<TypedArray> for Array {
    fn from(val: TypedArray) -> Array {
        Array::new(val.vec(), val.descriptor().clone())
    }
}
