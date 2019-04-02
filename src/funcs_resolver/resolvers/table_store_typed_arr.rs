use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::TypedArray;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableStoreTypedArrayResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableStoreTypedArrayResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // offset
                ValueType::I32, // elem_size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let offset: u32 = args.nth_checked(1)?;
        let elem_size: u32 = args.nth_checked(2)?;
        let mut val: TypedArray = target.memory().get_dyn_value(offset)?;
        val.resize(elem_size)?;
        target
            .table()
            .insert(key, val.into())
            .map(|_| None)
            .map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableStoreTypedArrayResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableStoreTypedArrayResolver {})
    }
}
