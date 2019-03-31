use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, Raw};
use crate::utils::map_trap::MapTrap;
use crate::StackVal;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

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
        let memory = target.memory().raw();
        let bytes = memory.get(offset, size as usize).map_trap()?;
        stack.push(Raw::new(bytes).into()).map(|_| None).map_trap()
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
