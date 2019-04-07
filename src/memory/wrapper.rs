use super::StorageValType;
use crate::utils::errors::{ErrInto, RuntimeError};
use alloc::prelude::*;
use wasmi::memory_units::{ByteSize, Pages};
use wasmi::{LittleEndianConvert, MemoryInstance, MemoryRef};

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
    fn written_size(&self) -> usize;
    fn chunks(&self) -> Vec<Vec<u8>> {
        self.vec()
            .chunks_exact(self.elem_size() as usize)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

pub trait DynLittleEndianConvert: MemoryVal + Sized {
    fn from_little_endian_info(
        memory: MemoryRef,
        offset: u32,
    ) -> Result<(u32, MemoryDescriptor), RuntimeError>;
    fn to_little_endian_info(&self) -> usize {
        self.written_size()
    }
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
