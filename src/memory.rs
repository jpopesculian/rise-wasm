use crate::stack_based_memory::{StackVal, StackValType};
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert;
use wasmi::memory_units::Pages;
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

    pub fn get_value<T: LittleEndianConvert>(&self, offset: u32) -> Result<T, Error> {
        self.borrow().get_value::<T>(offset)
    }

    pub fn set_value<T: LittleEndianConvert>(&self, offset: u32, value: T) -> Result<(), Error> {
        self.borrow().set_value::<T>(offset, value)
    }

    pub fn get_dyn_value<T: DynLittleEndianConvert>(&self, offset: u32) -> Result<T, Error> {
        let descriptor = self.get(offset, 12)?;
        let (new_offset, size) = T::get_from_info(&descriptor, offset)
            .map_err(|_| Error::Value("Invalid Descriptor".to_string()))?;
        let slice = self.get(new_offset, size)?;
        T::from_little_endian(&slice)
            .map_err(|_| Error::Value("Could not convert from little endian".to_string()))
    }

    pub fn set_dyn_value<T: DynLittleEndianConvert>(
        &self,
        offset: u32,
        value: T,
    ) -> Result<(), Error> {
        let size = value.get_to_info();
        let mut bytes = self.get(offset, size)?;
        value.into_little_endian(&mut bytes, offset);
        self.set(offset, &bytes)
    }

    pub fn get(&self, offset: u32, size: usize) -> Result<Vec<u8>, Error> {
        self.borrow().get(offset, size)
    }

    pub fn set(&self, offset: u32, value: &[u8]) -> Result<(), Error> {
        self.borrow().set(offset, value)
    }
}

pub trait MemoryVal {
    fn new(data: Vec<u8>) -> Self;
    fn bytes(&self) -> &[u8];
    fn vec(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
    fn string(&self) -> Result<String, String> {
        Err("Could not convert to String".to_string())
    }
    fn val_type() -> StackValType;
}

pub trait DynLittleEndianConvert: MemoryVal + Sized {
    fn get_from_info(buffer: &[u8], offset: u32) -> Result<(u32, usize), ValueError>;
    fn get_to_info(&self) -> usize;
    fn into_little_endian(self, buffer: &mut [u8], _offset: u32);
    fn from_little_endian(buffer: &[u8]) -> Result<Self, ValueError>;
}

pub struct Utf16String(Vec<u8>);

impl DynLittleEndianConvert for Utf16String {
    fn get_from_info(buffer: &[u8], offset: u32) -> Result<(u32, usize), ValueError> {
        let size = buffer
            .get(0..4)
            .map(|s| LittleEndian::read_u32(s))
            .ok_or(ValueError::InvalidLittleEndianBuffer)? as usize;
        Ok((offset + 4, size * 2))
    }

    fn get_to_info(&self) -> usize {
        self.bytes().len() + 4
    }

    fn into_little_endian(self, buffer: &mut [u8], _offset: u32) {
        let mut len_descriptor = [0; 4];
        LittleEndian::write_u32(&mut len_descriptor, (self.bytes().len() / 2) as u32);
        let mut result = len_descriptor.to_vec();
        result.append(&mut self.vec());
        buffer.copy_from_slice(&result);
    }

    fn from_little_endian(buffer: &[u8]) -> Result<Self, ValueError> {
        let mut result = Vec::<u8>::new();
        buffer.clone_into(&mut result);
        Ok(Utf16String::new(result))
    }
}

impl MemoryVal for Utf16String {
    fn new(data: Vec<u8>) -> Utf16String {
        Utf16String(data)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn string(&self) -> Result<String, String> {
        let len = self.bytes().len() / 2;
        let mut dest = Vec::<u16>::with_capacity(len);
        dest.resize(len, 0);
        LittleEndian::read_u16_into(self.bytes(), &mut dest);
        String::from_utf16(&dest).map_err(|_| "Failed to decode from utf16".to_string())
    }

    fn val_type() -> StackValType {
        StackValType::Utf16
    }
}

impl convert::TryFrom<Raw> for Utf16String {
    type Error = String;

    fn try_from(val: Raw) -> Result<Utf16String, Self::Error> {
        if val.bytes().len() % 2 != 0 {
            Err("Array does not have an even number of bytes".to_string())
        } else {
            Ok(Utf16String::new(val.vec()))
        }
    }
}

impl convert::From<Utf8String> for Utf16String {
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
        Utf16String::new(new_bytes)
    }
}

impl convert::TryFrom<StackVal> for Utf16String {
    type Error = String;

    fn try_from(val: StackVal) -> Result<Utf16String, Self::Error> {
        match val.val_type {
            StackValType::Utf16 => Ok(Utf16String::new(val.data)),
            StackValType::Utf8 => Ok(Utf8String::new(val.data).into()),
            StackValType::Raw => Utf16String::try_from(Raw::new(val.data)),
        }
    }
}

pub struct Utf8String(Vec<u8>);

impl DynLittleEndianConvert for Utf8String {
    fn get_from_info(buffer: &[u8], offset: u32) -> Result<(u32, usize), ValueError> {
        let size = buffer
            .get(0..4)
            .map(|s| LittleEndian::read_u32(s))
            .ok_or(ValueError::InvalidLittleEndianBuffer)? as usize;
        Ok((offset + 4, size))
    }

    fn get_to_info(&self) -> usize {
        self.bytes().len() + 4
    }

    fn into_little_endian(self, buffer: &mut [u8], _offset: u32) {
        let mut len_descriptor = [0; 4];
        LittleEndian::write_u32(&mut len_descriptor, self.bytes().len() as u32);
        let mut result = len_descriptor.to_vec();
        result.append(&mut self.vec());
        buffer.copy_from_slice(&result);
    }

    fn from_little_endian(buffer: &[u8]) -> Result<Self, ValueError> {
        let mut result = Vec::<u8>::new();
        buffer.clone_into(&mut result);
        Ok(Utf8String::new(result))
    }
}

impl MemoryVal for Utf8String {
    fn new(data: Vec<u8>) -> Utf8String {
        Utf8String(data)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn string(&self) -> Result<String, String> {
        String::from_utf8(self.bytes().to_vec())
            .map_err(|_| "Failed to decode from utf8".to_string())
    }

    fn val_type() -> StackValType {
        StackValType::Utf8
    }
}

impl convert::From<Raw> for Utf8String {
    fn from(val: Raw) -> Utf8String {
        Utf8String::new(val.vec())
    }
}

impl convert::TryFrom<StackVal> for Utf8String {
    type Error = String;

    fn try_from(val: StackVal) -> Result<Utf8String, Self::Error> {
        match val.val_type {
            StackValType::Utf16 => Err("Cannot convert from Utf8String".to_string()),
            StackValType::Utf8 => Ok(Utf8String::new(val.data)),
            StackValType::Raw => Ok(Raw::new(val.data).into()),
        }
    }
}

pub struct Raw(Vec<u8>);

impl MemoryVal for Raw {
    fn new(data: Vec<u8>) -> Raw {
        Raw(data)
    }

    fn bytes(&self) -> &[u8] {
        &self.0
    }

    fn val_type() -> StackValType {
        StackValType::Raw
    }
}

impl convert::From<Utf16String> for Raw {
    fn from(val: Utf16String) -> Raw {
        Raw::new(val.vec())
    }
}

impl convert::From<Utf8String> for Raw {
    fn from(val: Utf8String) -> Raw {
        Raw::new(val.vec())
    }
}

impl convert::From<StackVal> for Raw {
    fn from(val: StackVal) -> Raw {
        match val.val_type {
            StackValType::Utf16 => Utf16String::new(val.data).into(),
            StackValType::Utf8 => Utf8String::new(val.data).into(),
            StackValType::Raw => Raw::new(val.data),
        }
    }
}
