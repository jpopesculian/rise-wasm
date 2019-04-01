use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::Array;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct ArrToStackResolver;

impl<T: ResolverTarget> FuncResolver<T> for ArrToStackResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let stack = target.stack();
        let memory = target.memory();
        let val: Array = memory.get_dyn_value(offset).map_trap()?;
        stack.push(val.into()).map(|_| None).map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for ArrToStackResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(ArrToStackResolver {})
    }
}
