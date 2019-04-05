mod arena;
mod buddy;
mod uninitialized;

use crate::utils::errors::RuntimeError;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt;

pub use arena::ArenaAllocator;
pub use uninitialized::UninitializedAllocator;

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
