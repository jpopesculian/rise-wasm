use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, Raw, Utf16String};
use crate::utils::map_trap::MapTrap;
use crate::StackVal;
use alloc::prelude::*;
use core::convert::TryFrom;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct HexDecodeResolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let val: Utf16String = Utf16String::try_from(stack.pop().map_trap()?).map_trap()?;
        let string_rep = val.string().map_trap()?;
        let decoded = hex::decode(string_rep).map_trap()?;
        stack
            .push(Raw::new(decoded).into())
            .map(|_| None)
            .map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for HexDecodeResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(HexDecodeResolver {})
    }
}
