use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn verify_sig(sig: &[u8], pubkey: &[u8]) -> i32;
}

pub struct VerifySigResolver;

impl<T: ResolverTarget> FuncResolver<T> for VerifySigResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32))
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let start_sig: u32 = args.nth_checked(0)?;
        let start_pubkey: u32 = args.nth_checked(1)?;

        let sig = target
            .stack()
            .memory()
            .get(start_sig, 128)
            .expect("could not get memory");
        let pubkey = target
            .stack()
            .memory()
            .get(start_pubkey, 66)
            .expect("could not get memory");

        Ok(Some(RuntimeValue::I32(verify_sig(&sig, &pubkey))))
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
