use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, TypedArray};
use crate::utils::{js_buffer::JsBuffer, map_trap::MapTrap};
use alloc::prelude::*;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn hash160(bytes: &[u8]) -> JsValue;
}

pub struct Hash160Resolver;

impl<T: ResolverTarget> FuncResolver<T> for Hash160Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // src
                ValueType::I32, // dest
            ][..],
            Some(ValueType::I32), // size
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let src = args.nth_checked(0)?;
        let dest = args.nth_checked(1)?;
        let memory = target.memory();
        let val: TypedArray = memory.get_dyn_value(src).map_trap()?;
        let hash: JsBuffer = hash160(val.bytes()).into_serde().map_trap()?;
        let result = TypedArray::default(hash.to_vec());
        let size = memory.set_dyn_value(dest, result).map_trap()?;
        Ok(Some(RuntimeValue::I32(size as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for Hash160Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(Hash160Resolver {})
    }
}
