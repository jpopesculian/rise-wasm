use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn look(mem: &[u8]);
}

pub struct LookResolver;

impl<T: ResolverTarget> FuncResolver<T> for LookResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[ValueType::I32, ValueType::I32][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let offset: u32 = args.nth_checked(0)?;
        let len: u32 = args.nth_checked(1)?;
        look(
            &target
                .stack()
                .memory()
                .get(offset, len as usize)
                .expect("could not get memory"),
        );
        Ok(None)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for LookResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(LookResolver {})
    }
}
