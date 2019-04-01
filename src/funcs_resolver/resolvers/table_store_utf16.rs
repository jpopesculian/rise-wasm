use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::Utf16String;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableStoreUtf16Resolver;

impl<T: ResolverTarget> FuncResolver<T> for TableStoreUtf16Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // offset
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let offset: u32 = args.nth_checked(1)?;
        let val: Utf16String = target.memory().get_dyn_value(offset).map_trap()?;
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

impl<T: ResolverTarget> FuncResolverBuild<T> for TableStoreUtf16Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableStoreUtf16Resolver {})
    }
}
