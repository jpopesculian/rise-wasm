use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind};

pub struct StackDupResolver;

impl<T: ResolverTarget> FuncResolver<T> for StackDupResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let val = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        stack.push(val.clone()).map_trap(TrapKind::Unreachable)?;
        stack.push(val).map_trap(TrapKind::Unreachable)?;
        Ok(None)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for StackDupResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(StackDupResolver {})
    }
}
