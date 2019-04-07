use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use crate::memory::Utf8String;
use alloc::prelude::*;
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
        let utils = ResolverUtils::new(target, args);
        let val: Utf8String = utils.table_arg(0)?;
        Ok(Some(utils.send(val)?.into()))
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
