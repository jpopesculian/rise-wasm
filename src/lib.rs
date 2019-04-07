#![feature(
    alloc,
    associated_type_defaults,
    refcell_replace_swap,
    toowned_clone_into,
    optin_builtin_traits
)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate byteorder;
extern crate cfg_if;
extern crate hashbrown;
extern crate hex;
extern crate hex_serde;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate wasmi;

use alloc::prelude::*;
use serde::{Deserialize, Serialize};
use utils::panic_hook::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance};

mod funcs;
mod gas_middleware;
mod globals;
mod imports;
pub mod memory;
pub mod utils;

use funcs::build_funcs_resolver;
use gas_middleware::GasMiddleware;
use globals::Globals;
use imports::ImportResolver;
use memory::{MemoryWrapper, Raw, TableStorage};
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
    time: u64,
    args: Vec<JsBuffer>,
}

#[wasm_bindgen]
pub fn verify(wasm_binary: &[u8], options: &JsValue) {
    set_panic_hook();

    // parse_options
    let options: Options = options.into_serde().expect("Failed to parse options");

    // load and validate wasm
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("Failed to load wasm");

    // build memory
    let memory = MemoryWrapper::default();
    let table = TableStorage::default();
    for (i, arg) in options.args.iter().enumerate() {
        table
            .insert(i as u32, Raw::default(arg.to_vec()).into())
            .unwrap();
    }

    // build globals
    let globals = Globals::default()
        .with_global("now", options.time);

    // build resolvers
    let resolvers = build_funcs_resolver::<ImportResolver>();
    let mut externals = ImportResolver::new(
        resolvers.clone(),
        table.clone(),
        memory.clone(),
        globals.clone(),
    );
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
