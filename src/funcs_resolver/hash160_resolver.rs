use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn hash160(bytes: &[u8]) -> JsValue;
}

pub struct Hash160Resolver;

impl<T: ResolverTarget> FuncResolver<T> for Hash160Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[ValueType::I32, ValueType::I32][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let input: u32 = args.nth_checked(0)?;
        let output: u32 = args.nth_checked(1)?;
        let result = hash160(
            &target
                .memory()
                .get(input, 66)
                .expect("could not get memory"),
        )
        .as_string()
        .expect("Didn't receive string");
        target
            .memory()
            .set(output, result.as_bytes())
            .expect("Couldn't write to memory");
        Ok(None)
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for Hash160Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(Hash160Resolver {})
    }
}
