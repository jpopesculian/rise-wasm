use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use crate::utils::errors::RuntimeError;
use crate::memory::{Utf16String, MemoryVal};
use crate::utils::map_trap::MapTrap;
use core::convert::TryFrom;
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

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let msg_ptr = args.nth_checked(0)?;
        let file_ptr = args.nth_checked(1)?;
        let line: u32 = args.nth_checked(2)?;
        let column: u32 = args.nth_checked(3)?;
        let msg = target.memory().get_dyn_value::<Utf16String>(msg_ptr)?.string()?;
        let file = target.memory().get_dyn_value::<Utf16String>(file_ptr)?.string()?;
        Err(RuntimeError::new(format!("[{}: {},{}] {}", file, line, column, msg).to_string()).into())
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
