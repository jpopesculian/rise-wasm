use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{Utf8String, MemoryVal};
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
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let val: Utf8String = target.table().get(&key).map_trap()?.try_into()?;
        let dest = target.allocator().allocate(val.written_size() as u32)?;
        let _ = target.memory().set_dyn_value(dest, val)?;
        Ok(Some(RuntimeValue::I32(dest as i32)))
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
