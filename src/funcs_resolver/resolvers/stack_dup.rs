use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

pub struct StackDupResolver;

impl<T: ResolverTarget> FuncResolver<T> for StackDupResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let val = stack
            .pop()
            .ok_or(Trap::new(TrapKind::MemoryAccessOutOfBounds))?;
        stack
            .push(val.clone())
            .map_err(|_| Trap::new(TrapKind::Unreachable))?;
        stack
            .push(val)
            .map_err(|_| Trap::new(TrapKind::Unreachable))?;
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
