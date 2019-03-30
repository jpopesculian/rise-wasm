#![feature(alloc, associated_type_defaults)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate byteorder;
extern crate cfg_if;
extern crate hashbrown;
extern crate hex;
extern crate hex_serde;
extern crate serde;
extern crate wasm_bindgen;
extern crate wasmi;

use alloc::prelude::*;
use serde::{Deserialize, Serialize};
use utils::panic_hook::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance};

mod funcs_resolver;
mod gas_middleware;
mod imports;
mod stack_based_memory;
pub mod utils;

use funcs_resolver::build_funcs_resolver;
use gas_middleware::GasMiddleware;
use imports::ImportResolver;
pub use stack_based_memory::{StackBasedMemory, StackVal};
use utils::js_buffer::JsBuffer;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    name: String,
    args: Vec<JsBuffer>,
}

#[wasm_bindgen]
pub fn verify(wasm_binary: &[u8], options: &JsValue) {
    set_panic_hook();

    // parse_options
    let options: Options = options.into_serde().expect("Failed to parse options");

    // build memory
    let stack = StackBasedMemory::new();
    for arg in options.args {
        stack.push(StackVal::default(arg.clone())).unwrap();
    }

    // load and validate wasm
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("Failed to load wasm");

    // build resolvers
    let resolvers = build_funcs_resolver::<ImportResolver>();
    let mut externals = ImportResolver::new(resolvers.clone(), stack.clone());
    let imports = ImportsBuilder::new().with_resolver("env", &externals);

    // build module instance
    let instance = ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .push_middleware(Box::new(GasMiddleware::new(resolvers.clone())));

    if !instance.has_start() {
        panic!("Module does not have a start function defined")
    }

    instance
        .run_start(&mut externals)
        .expect("Failed to execute export");
}
