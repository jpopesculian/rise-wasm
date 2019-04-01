use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::TypedArray;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use core::convert::TryInto;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadTypedArrayResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadTypedArrayResolver {
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
        let val: TypedArray = target.table().get(&key).map_trap()?.try_into().map_trap()?;
        let size = target.memory().set_dyn_value(offset, val).map_trap()?;
        Ok(Some(RuntimeValue::I32(size as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadTypedArrayResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadTypedArrayResolver {})
    }
}
