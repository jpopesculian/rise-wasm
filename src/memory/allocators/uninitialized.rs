use super::{Allocator, AllocatorRef};
use crate::utils::errors::RuntimeError;
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Debug)]
pub struct UninitializedAllocator;

impl UninitializedAllocator {
    pub fn new() -> AllocatorRef {
        AllocatorRef(Rc::new(RefCell::new(UninitializedAllocator)))
    }
}

impl Allocator for UninitializedAllocator {
    fn allocate(&mut self, _: u32) -> Result<u32, RuntimeError> {
        Err(RuntimeError::new("Uninitialized Allocator"))
    }
    fn free(&mut self, _: u32) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn reset(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
