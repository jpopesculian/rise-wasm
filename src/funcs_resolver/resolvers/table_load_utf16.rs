use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::funcs_resolver::utils::ResolverUtils;
use crate::memory::Utf16String;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadUtf16Resolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadUtf16Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let val: Utf16String = utils.table_arg(0)?;
        Ok(Some(utils.send(val)?.into()))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadUtf16Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadUtf16Resolver {})
    }
}
