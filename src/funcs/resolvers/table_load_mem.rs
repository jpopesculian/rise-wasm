use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TableLoadMemResolver;

impl<T: ResolverTarget> FuncResolver<T> for TableLoadMemResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // key
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let key: u32 = args.nth_checked(0)?;
        let val = target.table().get(&key).map_trap()?;
        let dest = target.allocator().allocate(val.bytes().len() as u32)?;
        target.memory().set(dest, val.bytes())?;
        Ok(Some(RuntimeValue::I32(dest as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TableLoadMemResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TableLoadMemResolver {})
    }
}
