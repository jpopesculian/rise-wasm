use super::funcs_resolver::{FuncsResolverBuilder, ResolverTarget};
use alloc::prelude::*;
use alloc::rc::Rc;
use wasm_bindgen::prelude::*;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, MemoryRef, ModuleImportResolver, RuntimeArgs,
    RuntimeValue, Signature, Trap,
};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn log(x: usize);
}

pub struct ImportFuncs {
    memory: MemoryRef,
    resolvers: Rc<FuncsResolverBuilder<ImportFuncs>>,
}

impl Externals for ImportFuncs {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        self.resolvers.clone().run(self, index, args)
    }
}

impl ImportFuncs {
    pub fn new(resolvers: Rc<FuncsResolverBuilder<ImportFuncs>>, memory: MemoryRef) -> ImportFuncs {
        ImportFuncs { resolvers, memory }
    }
}

impl ResolverTarget for ImportFuncs {
    fn memory(&self) -> MemoryRef {
        self.memory.clone()
    }
}

pub struct ImportFuncsResolver {
    resolvers: Rc<FuncsResolverBuilder<ImportFuncs>>,
}

impl ImportFuncsResolver {
    pub fn new(resolvers: Rc<FuncsResolverBuilder<ImportFuncs>>) -> ImportFuncsResolver {
        ImportFuncsResolver { resolvers }
    }
}

impl ModuleImportResolver for ImportFuncsResolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        match self.resolvers.resolve(field_name) {
            (Some(index), Some(resolver)) => Ok(FuncInstance::alloc_host(
                resolver.signature(signature),
                index,
            )),
            _ => Err(Error::Function(String::from(
                "Could not find resolver at index",
            ))),
        }
    }
}
