use alloc::rc::Rc;

mod builder;
mod resolver;
mod resolvers;

pub use builder::{FuncResolverBuild, FuncsResolverBuilder};
pub use resolver::{FuncResolver, ResolverTarget};

use resolvers::*;

pub fn build_funcs_resolver<T: ResolverTarget>() -> Rc<FuncsResolverBuilder<T>> {
    Rc::new(
        FuncsResolverBuilder::<T>::new()
            .push("abort", AbortResolver::build())
            .push("compare", CompareResolver::build())
            .push("hash160", Hash160Resolver::build())
            .push("hex_decode", HexDecodeResolver::build())
            .push("mem_to_stack", MemToStackResolver::build())
            .push("stack_dup", StackDupResolver::build())
            .push("stack_to_mem", StackToMemResolver::build())
            .push("utf16_to_stack", Utf16ToStackResolver::build())
            .push("utf8_to_stack", Utf8ToStackResolver::build())
            .push("verify_sig", VerifySigResolver::build()),
    )
}
