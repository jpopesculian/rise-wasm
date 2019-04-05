use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::funcs_resolver::utils::ResolverUtils;
use crate::memory::Raw;
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
        let utils = ResolverUtils::new(target, args);
        let key: u32 = utils.arg(0)?;
        let offset: u32 = utils.arg(1)?;
        let size: u32 = utils.arg(2)?;
        let val = Raw::default(target.memory().get(offset, size as usize)?);
        utils.save(key, val)
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
