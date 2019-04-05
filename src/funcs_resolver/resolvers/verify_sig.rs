use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::funcs_resolver::utils::{ResolverUtils, RuntimeBool};
use crate::memory::{MemoryVal, TypedArray};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn verify_sig(sig: &[u8], pub_key: &[u8]) -> bool;
}

pub struct VerifySigResolver;

impl<T: ResolverTarget> FuncResolver<T> for VerifySigResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // signature
                ValueType::I32, // publicKey
            ][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        let signature: TypedArray = utils.mem_arg(0)?;
        let pubkey: TypedArray = utils.mem_arg(1)?;
        let is_verified = RuntimeBool::new(verify_sig(signature.bytes(), pubkey.bytes()));
        Ok(Some(is_verified.into()))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for VerifySigResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(VerifySigResolver {})
    }
}
