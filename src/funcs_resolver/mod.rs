mod builder;
mod resolver;

mod add_resolver;
mod compare_resolver;
mod hash160_resolver;
mod look_resolver;
mod minus_resolver;
mod verify_sig_resolver;

pub use builder::{FuncResolverBuild, FuncsResolverBuilder};
pub use resolver::{FuncResolver, ResolverTarget};

pub fn build_funcs_resolver<T: ResolverTarget>() -> FuncsResolverBuilder<T> {
    FuncsResolverBuilder::<T>::new()
        .push("add", add_resolver::AddResolver::build())
        .push("compare", compare_resolver::CompareResolver::build())
        .push("hash160", hash160_resolver::Hash160Resolver::build())
        .push("look", look_resolver::LookResolver::build())
        .push("minus", minus_resolver::MinusResolver::build())
        .push(
            "verify_sig",
            verify_sig_resolver::VerifySigResolver::build(),
        )
}
