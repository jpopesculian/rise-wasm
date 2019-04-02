use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, TypedArray};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn compare(bytes1: &[u8], bytes2: &[u8]) -> bool;
}

pub struct CompareResolver;

impl<T: ResolverTarget> FuncResolver<T> for CompareResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // left
                ValueType::I32, // right
            ][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let left_ptr = args.nth_checked(0)?;
        let right_ptr = args.nth_checked(1)?;
        let memory = target.memory();
        let left: TypedArray = memory.get_dyn_value(left_ptr)?;
        let right: TypedArray = memory.get_dyn_value(right_ptr)?;
        let is_equal = compare(left.bytes(), right.bytes());
        Ok(Some(RuntimeValue::I32(if is_equal { 1 } else { 0 })))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for CompareResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(CompareResolver {})
    }
}
