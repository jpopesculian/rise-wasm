use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadMemResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadMemResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // offset
            ][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let offset: u32 = args.nth_checked(1)?;
        let val = target.table().get(&key).map_trap()?;
        target.memory().set(offset, val.bytes()).map_trap()?;
        Ok(Some(RuntimeValue::I32(val.size() as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadMemResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadMemResolver {})
    }
}
