use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use crate::memory::Array;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableStoreArrayResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableStoreArrayResolver {
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
        let utils = ResolverUtils::new(target, args);
        let key = utils.arg(0)?;
        let val: Array = utils.mem_arg(1)?;
        utils.save(key, val)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableStoreArrayResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableStoreArrayResolver {})
    }
}
