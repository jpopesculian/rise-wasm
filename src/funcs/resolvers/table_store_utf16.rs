use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use crate::memory::Utf16String;
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
        let utils = ResolverUtils::new(target, args);
        let key: u32 = utils.arg(0)?;
        let val: Utf16String = utils.mem_arg(1)?;
        utils.save(key, val)
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
