use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::Raw;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableStoreMemResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableStoreMemResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // offset
                ValueType::I32, // size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let offset: u32 = args.nth_checked(1)?;
        let size: u32 = args.nth_checked(2)?;
        let bytes = target.memory().get(offset, size as usize).map_trap()?;
        target
            .table()
            .insert(key, Raw::default(bytes).into())
            .map(|_| None)
            .map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableStoreMemResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableStoreMemResolver {})
    }
}
