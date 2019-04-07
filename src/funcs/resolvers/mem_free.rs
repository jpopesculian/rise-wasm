use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct MemFreeResolver;

impl<T: ResolverTarget> FuncResolver<T> for MemFreeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // ptr
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let ptr = args.nth_checked(0)?;
        target.allocator().free(ptr)?;
        Ok(None)
    }

    fn gas(&self) -> u64 {
        0
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MemFreeResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MemFreeResolver {})
    }
}
