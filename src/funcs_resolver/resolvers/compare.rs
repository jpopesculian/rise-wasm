use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn compare(bytes1: &[u8], bytes2: &[u8]) -> bool;
}

pub struct CompareResolver;

impl<T: ResolverTarget> FuncResolver<T> for CompareResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], Some(ValueType::I32))
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let left = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        let right = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        let is_equal = compare(&left, &right);
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
