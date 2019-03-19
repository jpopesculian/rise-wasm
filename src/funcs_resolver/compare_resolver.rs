use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn compare(bytes1: &[u8], bytes2: &[u8]) -> i32;
}

pub struct CompareResolver;

impl<T: ResolverTarget> FuncResolver<T> for CompareResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[ValueType::I32, ValueType::I32, ValueType::I32][..],
            Some(ValueType::I32),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let start1: u32 = args.nth_checked(0)?;
        let start2: u32 = args.nth_checked(1)?;
        let len: u32 = args.nth_checked(2)?;

        let bytes1 = target
            .memory()
            .get(start1, len as usize)
            .expect("could not get memory");
        let bytes2 = target
            .memory()
            .get(start2, len as usize)
            .expect("could not get memory");

        Ok(Some(RuntimeValue::I32(compare(&bytes1, &bytes2))))
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for CompareResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(CompareResolver {})
    }
}
