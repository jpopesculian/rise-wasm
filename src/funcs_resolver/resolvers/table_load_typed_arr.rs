use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{TypedArray, MemoryVal};
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
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let val: TypedArray = target.table().get(&key).map_trap()?.try_into()?;
        let dest = target.allocator().allocate(val.written_size() as u32)?;
        let _ = target.memory().set_dyn_value(dest, val)?;
        Ok(Some(RuntimeValue::I32(dest as i32)))
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
