use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn minus(x: i32, y: i32) -> i32;
}

pub struct MinusResolver;

impl<T: ResolverTarget> FuncResolver<T> for MinusResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32))
    }

    fn run(&self, _: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let a: i32 = args.nth_checked(0)?;
        let b: i32 = args.nth_checked(1)?;
        let result = minus(a, b);
        Ok(Some(RuntimeValue::I32(result as i32)))
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for MinusResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(MinusResolver {})
    }
}
