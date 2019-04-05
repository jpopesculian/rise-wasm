use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::ArenaAllocator;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct MemInitArenaResolver;

impl<T: ResolverTarget> FuncResolver<T> for MemInitArenaResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // heap_offset
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let heap_offset = args.nth_checked(0)?;
        let max_offset = target.memory().max_offset();
        crate::log!("{} -> {}", heap_offset, max_offset);
        let allocator = ArenaAllocator::new(heap_offset, max_offset);
        target.set_allocator(allocator);
        Ok(None)
    }

    fn gas(&self) -> u64 {
        0
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MemInitArenaResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MemInitArenaResolver {})
    }
}
