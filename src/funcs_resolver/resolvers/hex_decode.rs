use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

pub struct HexDecodeResolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let encoded = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        let string_rep = String::from_utf8(encoded).map_trap(TrapKind::Unreachable)?;
        let decoded = hex::decode(string_rep).map_trap(TrapKind::Unreachable)?;
        stack
            .push(decoded)
            .map(|_| None)
            .map_trap(TrapKind::Unreachable)
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
