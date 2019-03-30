use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct StackToMemResolver;

impl<T: ResolverTarget> FuncResolver<T> for StackToMemResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
            ][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let stack = target.stack();
        let bytes = stack.pop().map_trap()?.data();
        stack.memory().set(offset, &bytes).map_trap()?;
        Ok(Some(RuntimeValue::I32(bytes.len() as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for StackToMemResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(StackToMemResolver {})
    }
}
