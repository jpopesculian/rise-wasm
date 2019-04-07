use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils, RuntimeBool};
use crate::memory::{Array, MemoryVal, TypedArray};
use crate::utils::{CollectResult, JsBuffer, MapTrap, RuntimeError};
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn verify_multi_sig(sigs: JsValue, pub_keys: JsValue) -> bool;
}

pub struct VerifyMultiSigResolver;

impl<T: ResolverTarget> FuncResolver<T> for VerifyMultiSigResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // signatures
                ValueType::I32, // publicKeys
            ][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let signatures: Vec<JsBuffer> = utils
            .multi_mem_arg::<TypedArray>(0)?
            .iter()
            .map(|key| key.into())
            .collect();
        let pubkeys: Vec<JsBuffer> = utils
            .multi_mem_arg::<TypedArray>(1)?
            .iter()
            .map(|key| key.into())
            .collect();
        let is_verified = RuntimeBool::new(verify_multi_sig(
            JsValue::from_serde(&signatures).map_trap()?,
            JsValue::from_serde(&pubkeys).map_trap()?,
        ));
        Ok(Some(is_verified.into()))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for VerifyMultiSigResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(VerifyMultiSigResolver {})
    }
}
