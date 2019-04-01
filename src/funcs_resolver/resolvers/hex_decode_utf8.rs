use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, TypedArray, Utf8String};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct HexDecodeUtf8Resolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeUtf8Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // src
                ValueType::I32, // dest
            ][..],
            Some(ValueType::I32), // size
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let src = args.nth_checked(0)?;
        let dest = args.nth_checked(1)?;
        let memory = target.memory();
        let val: Utf8String = memory.get_dyn_value(src).map_trap()?;
        let string_rep = val.string().map_trap()?;
        let decoded = TypedArray::default(hex::decode(string_rep).map_trap()?);
        let size = memory.set_dyn_value(dest, val).map_trap()?;
        Ok(Some(RuntimeValue::I32(size as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for HexDecodeUtf8Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(HexDecodeUtf8Resolver {})
    }
}
