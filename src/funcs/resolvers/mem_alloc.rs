use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct MemAllocResolver;

impl<T: ResolverTarget> FuncResolver<T> for MemAllocResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // size
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let size = args.nth_checked(0)?;
        let ptr = target.allocator().allocate(size)?;
        Ok(Some(RuntimeValue::I32(ptr as i32)))
    }

    fn gas(&self) -> u64 {
        0
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MemAllocResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MemAllocResolver {})
    }
}
