#![feature(alloc)]
#![no_std]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasmi;
extern crate alloc;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, RuntimeValue};

pub mod import_funcs;
pub mod import_globals;
pub mod import_memory;
pub mod utils;

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
    let module = wasmi::Module::from_buffer(&wasm_binary)
        .expect("failed to load wasm");

    // define imports
    let globals_resolver = import_globals::ImportGlobalsResolver::new(args.len() as i32);
    let mem_resolver = import_memory::ImportMemoryResolver::new(args);
    let funcs_resolver = import_funcs::ImportFuncsResolver::new();
    let imports = ImportsBuilder::new()
        .with_resolver("imports", &funcs_resolver)
        .with_resolver("memory", &mem_resolver)
        .with_resolver("globals", &globals_resolver);

    // build module instance
    let instance =  ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // set state
    let mut state = import_funcs::ImportFuncs::new(&mem_resolver);

    // call function and throw error if not equal to 1
    assert_eq!(
        instance.invoke_export("main", &[], &mut state)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1))
    )
}
