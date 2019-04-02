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
            .push("hex_decode_utf8", HexDecodeUtf8Resolver::build())
            .push("hex_decode_utf16", HexDecodeUtf16Resolver::build())
            .push("mem_alloc", MemAllocResolver::build())
            .push("mem_free", MemFreeResolver::build())
            .push("mem_reset", MemResetResolver::build())
            .push("mem_init_arena", MemInitArenaResolver::build())
            .push("table_load_arr", TableLoadArrayResolver::build())
            .push("table_load_mem", TableLoadMemResolver::build())
            .push("table_load_typed_arr", TableLoadTypedArrayResolver::build())
            .push("table_load_utf16", TableLoadUtf16Resolver::build())
            .push("table_load_utf8", TableLoadUtf8Resolver::build())
            .push("table_store_arr", TableStoreArrayResolver::build())
            .push("table_store_mem", TableStoreMemResolver::build())
            .push(
                "table_store_typed_arr",
                TableStoreTypedArrayResolver::build(),
            )
            .push("table_store_utf16", TableStoreUtf16Resolver::build())
            .push("table_store_utf8", TableStoreUtf8Resolver::build())
            .push("verify_sig", VerifySigResolver::build()),
    )
}
