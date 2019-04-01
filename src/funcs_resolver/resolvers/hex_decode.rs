use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, Raw, Utf8String};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use core::convert::TryInto;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct HexDecodeResolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let val: Utf8String = stack.pop().map_trap()?.try_into().map_trap()?;
        let string_rep = val.string().map_trap()?;
        let decoded = hex::decode(string_rep).map_trap()?;
        stack
            .push(Raw::default(decoded).into())
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
