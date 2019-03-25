use crate::StackBasedMemory;
use core::fmt;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub trait ResolverTarget {
    fn stack(self: &Self) -> StackBasedMemory;
}

pub trait FuncResolver<T> {
    fn signature(self: &Self, signature: &Signature) -> Signature;
    fn run(self: &Self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap>;
    fn gas(self: &Self) -> u64;
}

impl<T> fmt::Debug for FuncResolver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FuncResolver")
    }
}
