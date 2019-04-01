use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::TypedArray;
use crate::utils::map_trap::MapTrap;
use alloc::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct TypedArrToStackResolver;

impl<T: ResolverTarget> FuncResolver<T> for TypedArrToStackResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // offset
                ValueType::I32, // elem_size
            ][..],
            None,
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let elem_size: u32 = args.nth_checked(1)?;
        let memory = target.memory();
        let mut val: TypedArray = memory.get_dyn_value(offset).map_trap()?;
        crate::utils::log::log(&format!("{:?}", val));
        val.resize(elem_size).map_trap()?;
        crate::utils::log::log(&format!("{:?}", val));
        Ok(None)
        // let val: Array = memory.get_dyn_value(offset).map_trap()?;
        // stack.push(val.into()).map(|_| None).map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for TypedArrToStackResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(TypedArrToStackResolver {})
    }
}
