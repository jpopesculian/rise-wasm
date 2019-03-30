use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use crate::StackVal;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct Utf16ToStackResolver;

impl<T: ResolverTarget> FuncResolver<T> for Utf16ToStackResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let stack = target.stack();
        let size = LittleEndian::read_u32(&stack.memory().get(offset, 4).map_trap()?);
        let bytes = stack
            .memory()
            .get(offset + 4, (size * 2) as usize)
            .map_trap()?;
        stack.push(StackVal::utf16(bytes)).map(|_| None).map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for Utf16ToStackResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(Utf16ToStackResolver {})
    }
}
