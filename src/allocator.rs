use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::max;
use core::fmt;

pub trait Allocator: fmt::Debug {
    fn allocate(self: &mut Self, size: u32) -> Result<u32, RuntimeError>;
    fn free(self: &mut Self, ptr: u32) -> Result<(), RuntimeError>;
    fn reset(self: &mut Self) -> Result<(), RuntimeError>;
}

#[derive(Clone, Debug)]
pub struct AllocatorRef(Rc<RefCell<Allocator>>);

impl AllocatorRef {
    pub fn allocate(self: &mut Self, size: u32) -> Result<u32, RuntimeError> {
        self.0.borrow_mut().allocate(size)
    }
    pub fn free(self: &mut Self, ptr: u32) -> Result<(), RuntimeError> {
        self.0.borrow_mut().free(ptr)
    }
    pub fn reset(self: &mut Self) -> Result<(), RuntimeError> {
        self.0.borrow_mut().reset()
    }
}

#[derive(Debug)]
pub struct UnitializedAllocator;

impl UnitializedAllocator {
    pub fn new() -> AllocatorRef {
        AllocatorRef(Rc::new(RefCell::new(UnitializedAllocator)))
    }
}

impl Allocator for UnitializedAllocator {
    fn allocate(&mut self, size: u32) -> Result<u32, RuntimeError> {
        Err(RuntimeError::new("Unitialized Allocator"))
    }
    fn free(&mut self, ptr: u32) -> Result<(), RuntimeError> {
        Ok(())
    }
    fn reset(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ArenaAllocator {
    start_offset: u32,
    offset: u32,
    max_offset: u32,
}

impl ArenaAllocator {
    pub fn new(heap_base: u32, max_offset: u32) -> AllocatorRef {
        let start_offset = ArenaAllocator::align(heap_base);
        AllocatorRef(Rc::new(RefCell::new(ArenaAllocator {
            start_offset,
            offset: start_offset,
            max_offset,
        })))
    }

    fn align(offset: u32) -> u32 {
        let mask = (1 << 3) - 1;
        (offset + mask) & !mask
    }
}

impl Allocator for ArenaAllocator {
    fn allocate(&mut self, size: u32) -> Result<u32, RuntimeError> {
        let shift = max(size, 1);
        let offset = self.offset;
        let new_offset = ArenaAllocator::align(offset + shift);
        if new_offset > self.max_offset {
            Err(RuntimeError::new("Heap out of memory"))
        } else {
            self.offset = new_offset;
            Ok(offset)
        }
    }

    fn free(&mut self, ptr: u32) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
