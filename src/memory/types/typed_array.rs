use super::{Array, Raw, Utf16String, Utf8String};
use crate::memory::{
    DynLittleEndianConvert, MemoryDescriptor, MemoryVal, StorageVal, StorageValType,
};
use crate::utils::errors::RuntimeError;
use crate::utils::js_buffer::JsBuffer;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasmi::MemoryRef;

#[derive(Debug, Clone)]
pub struct TypedArray(Vec<u8>, MemoryDescriptor);

impl TypedArray {
    // pub fn with_elem_size(data: Vec<u8>, elem_size: u32) -> Result<TypedArray, RuntimeError> {
    //     let length = data.len() as u32;
    //     TypedArray::try_resizable(length, elem_size)?;
    //     Ok(TypedArray::new(
    //         data,
    //         MemoryDescriptor { length, elem_size },
    //     ))
    // }

    pub fn resize(&mut self, elem_size: u32) -> Result<(), RuntimeError> {
        TypedArray::try_resizable(self.size(), elem_size)?;
        self.1 = MemoryDescriptor {
            length: self.size() / elem_size,
            elem_size,
        };
        Ok(())
    }

    fn try_resizable(length: u32, elem_size: u32) -> Result<(), RuntimeError> {
        if TypedArray::valid_elem_size(elem_size) {
            if length % elem_size == 0 {
                Ok(())
            } else {
                Err(RuntimeError::new("Data cannot be split into element size"))
            }
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }

    fn valid_elem_size(elem_size: u32) -> bool {
        match elem_size {
            1 | 2 | 4 | 8 => true,
            _ => false,
        }
    }

    pub fn default(data: Vec<u8>) -> TypedArray {
        let length = data.len() as u32;
        TypedArray::new(
            data,
            MemoryDescriptor {
                length,
                elem_size: 1,
            },
        )
    }
}

impl MemoryVal for TypedArray {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> TypedArray {
        TypedArray(data, descriptor)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::TypedArray
    }

    fn written_size(&self) -> usize {
        self.bytes().len() + 20
    }
}

impl DynLittleEndianConvert for TypedArray {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError> {
        let arr_offset = memory.get(offset, 4).map(|s| LittleEndian::read_u32(&s))?;
        let byte_offset = memory
            .get(offset + 4, 4)
            .map(|s| LittleEndian::read_u32(&s))?;
        let length = memory
            .get(offset + 8, 4)
            .map(|s| LittleEndian::read_u32(&s))?;
        let descriptor = MemoryDescriptor {
            length,
            elem_size: 1,
        };
        Ok((arr_offset + byte_offset + 8, descriptor))
    }

    fn into_little_endian(self, buffer: &mut [u8], offset: u32) {
        let mut offset_descriptor = [0; 4];
        let mut size_descriptor = [0; 4];
        let alignment = [0; 4];
        LittleEndian::write_u32(&mut size_descriptor, self.size());
        LittleEndian::write_u32(&mut offset_descriptor, offset + 12);

        let mut result = offset_descriptor.to_vec();
        result.append(&mut alignment.to_vec());
        result.append(&mut size_descriptor.to_vec());
        result.append(&mut size_descriptor.to_vec());
        result.append(&mut alignment.to_vec());
        result.append(&mut self.vec());
        buffer.copy_from_slice(&result);
    }
}

impl TryFrom<StorageVal> for TypedArray {
    type Error = RuntimeError;

    fn try_from(val: StorageVal) -> Result<TypedArray, Self::Error> {
        match val.val_type {
            StorageValType::Utf16 => TypedArray::try_from(Utf16String::try_from(val)?),
            StorageValType::Utf8 => TypedArray::try_from(Utf8String::try_from(val)?),
            StorageValType::Raw => TypedArray::try_from(Raw::from(val)),
            StorageValType::Array => TypedArray::try_from(Array::try_from(val)?),
            StorageValType::TypedArray => Ok(TypedArray::new(val.data, val.descriptor)),
        }
    }
}

impl TryFrom<Raw> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Raw) -> Result<TypedArray, Self::Error> {
        TypedArray::try_resizable(val.size(), val.elem_size())?;
        Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
    }
}

impl TryFrom<Utf8String> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Utf8String) -> Result<TypedArray, Self::Error> {
        TypedArray::try_resizable(val.size(), val.elem_size())?;
        Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
    }
}

impl TryFrom<Utf16String> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Utf16String) -> Result<TypedArray, Self::Error> {
        TypedArray::try_resizable(val.size(), val.elem_size())?;
        Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
    }
}

impl TryFrom<Array> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Array) -> Result<TypedArray, Self::Error> {
        TypedArray::try_resizable(val.size(), val.elem_size())?;
        Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
    }
}

impl TryFrom<JsValue> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: JsValue) -> Result<TypedArray, Self::Error> {
        Ok(TypedArray::default(
            val.into_serde::<JsBuffer>()
                .map_err(RuntimeError::new)?
                .to_vec(),
        ))
    }
}
