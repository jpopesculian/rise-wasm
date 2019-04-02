use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::memory::{MemoryVal, TypedArray};
use crate::utils::map_trap::MapTrap;
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
        let sig_ptr = args.nth_checked(0)?;
        let pubkey_ptr = args.nth_checked(1)?;
        let memory = target.memory();
        let pubkey: TypedArray = memory.get_dyn_value(pubkey_ptr).map_trap()?;
        let sig: TypedArray = memory.get_dyn_value(sig_ptr).map_trap()?;
        let is_verified = verify_sig(sig.bytes(), pubkey.bytes());
        Ok(Some(RuntimeValue::I32(if is_verified { 1 } else { 0 })))
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
