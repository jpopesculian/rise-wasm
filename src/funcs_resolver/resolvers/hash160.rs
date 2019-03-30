use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::{js_buffer::JsBuffer, map_trap::MapTrap};
use crate::StackVal;
use alloc::prelude::*;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn hash160(bytes: &[u8]) -> JsValue;
}

pub struct Hash160Resolver;

impl<T: ResolverTarget> FuncResolver<T> for Hash160Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let value = stack.pop().map_trap()?;
        let hash: JsBuffer = hash160(&value.data()).into_serde().map_trap()?;
        stack.push(StackVal::default(hash.to_vec())).map_trap()?;
        Ok(None)
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
