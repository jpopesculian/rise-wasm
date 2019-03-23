use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

pub struct HexMemToStackResolver;

impl<T: ResolverTarget> FuncResolver<T> for HexMemToStackResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
                ValueType::I32, // size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let size: u32 = args.nth_checked(1)?;
        let stack = target.stack();
        let value = stack
            .memory()
            .get(offset, size as usize)
            .map_err(|_| Trap::new(TrapKind::MemoryAccessOutOfBounds))?;
        let string = String::from_utf8(value).map_err(|_| Trap::new(TrapKind::Unreachable))?;
        let bytes = hex::decode(string).map_err(|_| Trap::new(TrapKind::Unreachable))?;
        stack
            .push(bytes)
            .map(|_| None)
            .map_err(|_| Trap::new(TrapKind::Unreachable))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for HexMemToStackResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(HexMemToStackResolver {})
    }
}
