use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use crate::memory::{TypedArray, Utf16String};
use crate::utils::MapTrap;
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct HexDecodeUtf16Resolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeUtf16Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // src
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let val: Utf16String = utils.mem_arg(0)?;
        let decoded = TypedArray::default(hex::decode(val.string()?).map_trap()?);
        Ok(Some(utils.send(decoded)?.into()))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for HexDecodeUtf16Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(HexDecodeUtf16Resolver {})
    }
}
