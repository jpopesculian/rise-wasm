use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::funcs_resolver::utils::ResolverUtils;
use crate::memory::TypedArray;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableStoreTypedArrayResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableStoreTypedArrayResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
                ValueType::I32, // ptr
                ValueType::I32, // elem_size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let key: u32 = utils.arg(0)?;
        let mut val: TypedArray = utils.mem_arg(1)?;
        let elem_size: u32 = utils.arg(2)?;
        val.resize(elem_size)?;
        utils.save(key, val)
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
