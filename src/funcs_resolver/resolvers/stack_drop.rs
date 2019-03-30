use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct StackDropResolver;

impl<T: ResolverTarget> FuncResolver<T> for StackDropResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let _ = target.stack().pop().map_trap()?;
        Ok(None)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for StackDropResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(StackDropResolver {})
    }
}
