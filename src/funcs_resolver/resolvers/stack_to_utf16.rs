use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct StackToUtf16Resolver;

impl<T: ResolverTarget> FuncResolver<T> for StackToUtf16Resolver {
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
        let memory = stack.memory();

        let val = stack.pop().map_trap()?.to_utf16().map_trap()?;
        let mut len_descriptor = [0; 4];
        LittleEndian::write_u32(&mut len_descriptor, val.len() as u32);

        memory.set(offset, &len_descriptor).map_trap()?;
        memory.set(offset + 4, &val.data()).map_trap()?;

        Ok(None)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for StackToUtf16Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(StackToUtf16Resolver {})
    }
}
