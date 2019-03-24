#![feature(alloc, associated_type_defaults)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate cfg_if;
extern crate hashbrown;
extern crate hex;
extern crate hex_serde;
extern crate serde;
extern crate wasm_bindgen;
extern crate wasmi;

use alloc::prelude::*;
use cfg_if::cfg_if;
use core::ops::Deref;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, RuntimeValue};

mod funcs_resolver;
pub mod import_funcs;
pub mod import_globals;
pub mod import_memory;
mod stack_based_memory;
pub mod utils;

use funcs_resolver::build_funcs_resolver;
use stack_based_memory::StackBasedMemory;
use utils::js_buffer::JsBuffer;

#[derive(Serialize, Deserialize)]
pub struct Options {
    name: String,
    args: Vec<JsBuffer>,
}

#[wasm_bindgen]
pub fn verify(wasm_binary: &[u8], options: &JsValue) {
    // parse_options
    let options: Options = options.into_serde().unwrap();

    // build memory
    let stack = StackBasedMemory::new();
    for arg in options.args {
        stack.push(arg.clone()).unwrap();
    }

    // load and validate wasm
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    let resolvers = build_funcs_resolver::<import_funcs::ImportFuncs>();

    // let memory = StackBasedMemory::new();

    // define imports
    let mem_resolver = import_memory::ImportMemoryResolver::new(stack.memory());
    let globals_resolver = import_globals::ImportGlobalsResolver::new();
    let funcs_resolver = import_funcs::ImportFuncsResolver::new(resolvers.clone());
    let imports = ImportsBuilder::new()
        .with_resolver("imports", &funcs_resolver)
        .with_resolver("memory", &mem_resolver)
        .with_resolver("globals", &globals_resolver);

    // build module instance
    let instance = ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    let mut externals = import_funcs::ImportFuncs::new(resolvers.clone(), stack.clone());

    // call function and throw error if not equal to 1
    assert_eq!(
        instance
            .invoke_export(&options.name, &[], &mut externals)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1))
    );
}
