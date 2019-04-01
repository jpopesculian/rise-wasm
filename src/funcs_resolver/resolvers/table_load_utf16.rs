use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::Utf16String;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use core::convert::TryInto;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadUtf16Resolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadUtf16Resolver {
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
        let val: Utf16String = target.table().get(&key).map_trap()?.try_into().map_trap()?;
        let size = target.memory().set_dyn_value(offset, val).map_trap()?;
        Ok(Some(RuntimeValue::I32(size as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadUtf16Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadUtf16Resolver {})
    }
}
