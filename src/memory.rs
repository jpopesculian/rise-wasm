use crate::storage::{StorageVal, StorageValType};
use crate::utils::errors::{ErrInto, RuntimeError};
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasmi::memory_units::{ByteSize, Pages};
use wasmi::{Error, LittleEndianConvert, MemoryInstance, MemoryRef, ValueError};

#[derive(Debug, Clone)]
pub struct MemoryWrapper(MemoryRef);

impl MemoryWrapper {
    pub fn default() -> MemoryWrapper {
        MemoryWrapper(MemoryWrapper::build_memory())
    }

    fn build_memory() -> MemoryRef {
        MemoryInstance::alloc(Pages(1), Some(Pages(1))).expect("Memory could not be initialized")
    }

    pub fn raw(&self) -> MemoryRef {
        self.borrow().clone()
    }

    pub fn borrow(&self) -> &MemoryRef {
        &self.0
    }

    pub fn get_value<T: LittleEndianConvert>(&self, offset: u32) -> Result<T, RuntimeError> {
        self.borrow().get_value::<T>(offset).err_into()
    }

    pub fn set_value<T: LittleEndianConvert>(
        &self,
        offset: u32,
        value: T,
    ) -> Result<(), RuntimeError> {
        self.borrow().set_value::<T>(offset, value).err_into()
    }

    pub fn get_dyn_value<T: DynLittleEndianConvert>(&self, offset: u32) -> Result<T, RuntimeError> {
        let (new_offset, descriptor) = T::from_little_endian_info(self.raw(), offset)?;
        let size = (descriptor.length * descriptor.elem_size) as usize;
        let slice = self.get(new_offset, size)?;
        T::from_little_endian(&slice, descriptor)
            .map_err(|_| RuntimeError::new("Could not convert from little endian"))
    }

    pub fn set_dyn_value<T: DynLittleEndianConvert>(
        &self,
        offset: u32,
        value: T,
    ) -> Result<u32, RuntimeError> {
        let size = value.to_little_endian_info();
        let mut bytes = self.get(offset, size)?;
        value.into_little_endian(&mut bytes, offset);
        self.set(offset, &bytes)?;
        Ok(size as u32)
    }

    pub fn get(&self, offset: u32, size: usize) -> Result<Vec<u8>, RuntimeError> {
        self.borrow().get(offset, size).err_into()
    }

    pub fn set(&self, offset: u32, value: &[u8]) -> Result<(), RuntimeError> {
        self.borrow().set(offset, value).err_into()
    }

    pub fn max_offset(&self) -> u32 {
        (Pages::byte_size().0 * self.raw().current_size().0) as u32
    }
}

#[derive(Debug, Clone)]
pub struct MemoryDescriptor {
    pub length: u32,
    pub elem_size: u32,
}

pub trait MemoryVal {
    fn new(data: Vec<u8>, descriptor: MemoryDescriptor) -> Self;
    fn val_type() -> StorageValType;
    fn bytes(&self) -> &[u8];
    fn vec(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
    fn string(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::new("Could not convert to String"))
    }
    fn descriptor(&self) -> &MemoryDescriptor;
    fn len(&self) -> u32 {
        self.descriptor().length
    }
    fn elem_size(&self) -> u32 {
        self.descriptor().elem_size
    }
    fn size(&self) -> u32 {
        self.len() * self.elem_size()
    }
}

pub trait DynLittleEndianConvert: MemoryVal + Sized {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError>;
    fn to_little_endian_info(&self) -> usize;
    fn into_little_endian(self, buffer: &mut [u8], _offset: u32);
    fn from_little_endian(
        buffer: &[u8],
        descriptor: MemoryDescriptor,
    ) -> Result<Self, RuntimeError> {
        let mut result = Vec::<u8>::new();
        buffer.clone_into(&mut result);
        Ok(Self::new(result, descriptor))
    }
}

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

    fn to_little_endian_info(&self) -> usize {
        self.bytes().len() + 4
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

    fn string(&self) -> Result<String, RuntimeError> {
        let len = self.bytes().len() / 2;
        let mut dest = Vec::<u16>::with_capacity(len);
        dest.resize(len, 0);
        LittleEndian::read_u16_into(self.bytes(), &mut dest);
        String::from_utf16(&dest).map_err(|_| RuntimeError::new("Failed to decode from utf16"))
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Utf16
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

    fn to_little_endian_info(&self) -> usize {
        self.bytes().len() + 4
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

    fn string(&self) -> Result<String, RuntimeError> {
        String::from_utf8(self.bytes().to_vec())
            .map_err(|_| RuntimeError::new("Failed to decode from utf8"))
    }

    fn descriptor(&self) -> &MemoryDescriptor {
        &self.1
    }

    fn val_type() -> StorageValType {
        StorageValType::Utf8
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

#[derive(Debug, Clone)]
pub struct Array(Vec<u8>, MemoryDescriptor);

impl Array {
    pub fn with_elem_size(data: Vec<u8>, elem_size: u32) -> Result<Array, String> {
        let length = data.len() as u32;
        if length % elem_size != 0 {
            Err("Data cannot be split into element size".to_string())
        } else {
            Ok(Array::new(data, MemoryDescriptor { length, elem_size }))
        }
    }

    pub fn default(data: Vec<u8>) -> Array {
        let length = data.len() as u32;
        Array::new(
            data,
            MemoryDescriptor {
                length,
                elem_size: 1,
            },
        )
    }
}

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

    fn to_little_endian_info(&self) -> usize {
        self.bytes().len() + 16
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

#[derive(Debug, Clone)]
pub struct TypedArray(Vec<u8>, MemoryDescriptor);

impl TypedArray {
    pub fn with_elem_size(data: Vec<u8>, elem_size: u32) -> Result<TypedArray, RuntimeError> {
        let length = data.len() as u32;
        if TypedArray::valid_elem_size(elem_size) {
            if length % elem_size != 0 {
                Err(RuntimeError::new("Data cannot be split into element size"))
            } else {
                Ok(TypedArray::new(
                    data,
                    MemoryDescriptor { length, elem_size },
                ))
            }
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }

    pub fn resize(&mut self, elem_size: u32) -> Result<(), RuntimeError> {
        if TypedArray::valid_elem_size(elem_size) {
            if self.size() % elem_size != 0 {
                Err(RuntimeError::new("Data cannot be split into element size"))
            } else {
                self.1 = MemoryDescriptor {
                    length: self.size() / elem_size,
                    elem_size,
                };
                Ok(())
            }
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }

    pub fn valid_elem_size(elem_size: u32) -> bool {
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
        // TODO fix this
        StorageValType::Array
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

    fn to_little_endian_info(&self) -> usize {
        self.bytes().len() + 20
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
        if TypedArray::valid_elem_size(val.descriptor().elem_size) {
            Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }
}

impl TryFrom<Utf8String> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Utf8String) -> Result<TypedArray, Self::Error> {
        if TypedArray::valid_elem_size(val.descriptor().elem_size) {
            Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }
}

impl TryFrom<Utf16String> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Utf16String) -> Result<TypedArray, Self::Error> {
        if TypedArray::valid_elem_size(val.descriptor().elem_size) {
            Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }
}

impl TryFrom<Array> for TypedArray {
    type Error = RuntimeError;
    fn try_from(val: Array) -> Result<TypedArray, Self::Error> {
        if TypedArray::valid_elem_size(val.descriptor().elem_size) {
            Ok(TypedArray::new(val.vec(), val.descriptor().clone()))
        } else {
            Err(RuntimeError::new("Elem size must be a multiple of 2"))
        }
    }
}

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
