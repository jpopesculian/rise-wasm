use super::{Allocator, AllocatorRef};
use crate::utils::errors::RuntimeError;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::cmp::max;

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

    fn free(&mut self, _: u32) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), RuntimeError> {
        self.offset = self.start_offset;
        Ok(())
    }
}
