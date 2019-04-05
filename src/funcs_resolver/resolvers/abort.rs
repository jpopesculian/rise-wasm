use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::funcs_resolver::utils::ResolverUtils;
use crate::memory::Utf16String;
use crate::utils::errors::RuntimeError;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

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

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let msg: Utf16String = utils.mem_arg(0)?;
        let file: Utf16String = utils.mem_arg(1)?;
        let line: u32 = utils.arg(2)?;
        let column: u32 = utils.arg(3)?;
        Err(RuntimeError::new(
            format!(
                "[{}: {},{}] {}",
                file.string()?,
                line,
                column,
                msg.string()?
            )
            .to_string(),
        )
        .into())
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
