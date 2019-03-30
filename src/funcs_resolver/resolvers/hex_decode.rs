use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::utils::map_trap::MapTrap;
use crate::StackVal;
use alloc::prelude::*;
use hex;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct HexDecodeResolver;

impl<T: ResolverTarget> FuncResolver<T> for HexDecodeResolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, _: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let string_rep = stack.pop().map_trap()?.string().map_trap()?;
        let decoded = hex::decode(string_rep).map_trap()?;
        stack
            .push(StackVal::default(decoded))
            .map(|_| None)
            .map_trap()
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for HexDecodeResolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(HexDecodeResolver {})
    }
}
