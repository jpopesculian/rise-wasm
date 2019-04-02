use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct MemResetResolver;

impl<T: ResolverTarget> FuncResolver<T> for MemResetResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        target.allocator().reset()?;
        Ok(None)
    }

    fn gas(&self) -> u64 {
        0
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MemResetResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MemResetResolver {})
    }
}
