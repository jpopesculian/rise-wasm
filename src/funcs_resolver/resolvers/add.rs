use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use alloc::prelude::Box;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub struct AddResolver;

impl<T: ResolverTarget> FuncResolver<T> for AddResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32))
    }

    fn run(&self, _: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let a: i32 = args.nth_checked(0)?;
        let b: i32 = args.nth_checked(1)?;
        let result = a + b;
        Ok(Some(RuntimeValue::I32(result as i32)))
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for AddResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(AddResolver {})
    }
}
