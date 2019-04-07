use super::{FuncResolver, FuncResolverBuild, ResolverTarget, ResolverUtils};
use alloc::prelude::*;
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

#[wasm_bindgen(module = "./imports")]
extern "C" {
    fn epoch_time(
        year: u32,
        month: u32,
        day: u32,
        hours: u32,
        minutes: u32,
        seconds: u32,
    ) -> i64;
}

pub struct EpochTimeResolver;

impl<T: ResolverTarget> FuncResolver<T> for EpochTimeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(
            &[
                ValueType::I32, // year
                ValueType::I32, // month
                ValueType::I32, // day
                ValueType::I32, // hours
                ValueType::I32, // minutes
                ValueType::I32, // seconds
            ][..],
            Some(ValueType::I64),
        )
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let utils = ResolverUtils::new(target, args);
        Ok(Some(
            epoch_time(
                utils.arg(0)?,
                utils.arg(1)?,
                utils.arg(2)?,
                utils.arg(3)?,
                utils.arg(4)?,
                utils.arg(5)?,
            )
            .into(),
        ))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for EpochTimeResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(EpochTimeResolver {})
    }
}
