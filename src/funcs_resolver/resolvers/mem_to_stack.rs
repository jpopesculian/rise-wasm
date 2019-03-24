use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

pub struct MemToStackResolver;

impl<T: ResolverTarget> FuncResolver<T> for MemToStackResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
                ValueType::I32, // size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let size: u32 = args.nth_checked(1)?;
        let stack = target.stack();
        let bytes = stack
            .memory()
            .get(offset, size as usize)
            .map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        stack
            .push(bytes)
            .map(|_| None)
            .map_trap(TrapKind::Unreachable)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MemToStackResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MemToStackResolver {})
    }
}
