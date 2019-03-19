#![feature(alloc, associated_type_defaults)]
#![no_std]

extern crate alloc;
extern crate cfg_if;
extern crate hashbrown;
extern crate wasm_bindgen;
extern crate wasmi;

use alloc::rc::Rc;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, RuntimeValue};

mod funcs_resolver;
pub mod import_funcs;
pub mod import_globals;
pub mod import_memory;
pub mod utils;

use funcs_resolver::{build_funcs_resolver, FuncsResolverBuilder};

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn verify(wasm_binary: &[u8], args: &[u8]) {
    // load and validate wasm
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    let resolvers: Rc<FuncsResolverBuilder<import_funcs::ImportFuncs>> =
        Rc::new(build_funcs_resolver());

    // define imports
    let mem_resolver = import_memory::ImportMemoryResolver::new(args);
    let globals_resolver = import_globals::ImportGlobalsResolver::new(args.len() as i32);
    let funcs_resolver = import_funcs::ImportFuncsResolver::new(Rc::clone(&resolvers));
    let imports = ImportsBuilder::new()
        .with_resolver("imports", &funcs_resolver)
        .with_resolver("memory", &mem_resolver)
        .with_resolver("globals", &globals_resolver);

    // build module instance
    let instance = ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    let mut externals =
        import_funcs::ImportFuncs::new(Rc::clone(&resolvers), mem_resolver.memory.clone());

    // call function and throw error if not equal to 1
    assert_eq!(
        instance
            .invoke_export("main", &[], &mut externals)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1))
    );
}
