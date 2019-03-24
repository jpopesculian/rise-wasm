use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn verify_sig(sig: &[u8], pub_key: &[u8]) -> bool;
}

pub struct VerifySigResolver;

impl<T: ResolverTarget> FuncResolver<T> for VerifySigResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], Some(ValueType::I32))
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let pub_key = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        let sig = stack.pop().map_trap(TrapKind::MemoryAccessOutOfBounds)?;
        let is_verified = verify_sig(&sig, &pub_key);
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
