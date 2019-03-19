use wasmi::{MemoryRef, RuntimeArgs, RuntimeValue, Signature, Trap};

pub trait ResolverTarget {
    fn memory(self: &Self) -> MemoryRef;
}

pub trait FuncResolver<T> {
    fn signature(self: &Self, signature: &Signature) -> Signature;
    fn run(self: &Self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap>;
}
