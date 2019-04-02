use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

pub struct AbortResolver;

impl<T: ResolverTarget> FuncResolver<T> for AbortResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // msg
                ValueType::I32, // file
                ValueType::I32, // line
                ValueType::I32, // column
            ][..],
            None,
        )
    }

    fn run(&self, _: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        Err(Trap::new(TrapKind::Unreachable))
    }

    fn gas(&self) -> u64 {
        0
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for AbortResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(AbortResolver {})
    }
}
