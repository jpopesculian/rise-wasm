use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::Utf8String;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use core::convert::TryInto;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadUtf8Resolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadUtf8Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // offset
            ][..],
            Some(ValueType::I32), // size
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let offset: u32 = args.nth_checked(1)?;
        let val: Utf8String = target.table().get(&key).map_trap()?.try_into()?;
        let size = target.memory().set_dyn_value(offset, val)?;
        Ok(Some(RuntimeValue::I32(size as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadUtf8Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadUtf8Resolver {})
    }
}
