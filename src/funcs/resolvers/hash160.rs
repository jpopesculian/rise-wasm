use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use crate::memory::{MemoryVal, TypedArray};
use alloc::prelude::*;
use core::convert::TryInto;
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
            ][..],
            Some(ValueType::I32), // ptr
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let val: TypedArray = utils.mem_arg(0)?;
        let hash: TypedArray = hash160(val.bytes()).try_into()?;
        Ok(Some(utils.send(hash)?.into()))
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
