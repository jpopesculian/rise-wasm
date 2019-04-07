use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils, RuntimeBool};
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
        let utils = ResolverUtils::new(target, args);
        let left: TypedArray = utils.mem_arg(0)?;
        let right: TypedArray = utils.mem_arg(1)?;
        let is_equal: RuntimeBool = compare(left.bytes(), right.bytes()).into();
        Ok(Some(is_equal.into()))
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
