use crate::allocator::AllocatorRef;
use crate::memory::MemoryWrapper;
use crate::storage::TableStorage;
use core::fmt;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub trait ResolverTarget {
    fn memory(self: &Self) -> MemoryWrapper;
    fn table(self: &Self) -> TableStorage;
    fn allocator(self: &Self) -> AllocatorRef;
    fn set_allocator(self: &mut Self, allocator: AllocatorRef);
}

pub trait FuncResolver<T> {
    fn signature(self: &Self, signature: &Signature) -> Signature;
    fn run(self: &Self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap>;
    fn gas(self: &Self) -> u64;
}

impl<T> fmt::Debug for FuncResolver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FuncResolver")
    }
}
