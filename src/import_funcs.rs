use super::funcs_resolver::{FuncsResolverBuilder, ResolverTarget};
use super::StackBasedMemory;
use alloc::prelude::*;
use alloc::rc::Rc;
use wasm_bindgen::prelude::*;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue,
    Signature, Trap,
};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn log(x: usize);
}

pub struct ImportFuncs {
    stack: StackBasedMemory,
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
    pub fn new(
        resolvers: Rc<FuncsResolverBuilder<ImportFuncs>>,
        stack: StackBasedMemory,
    ) -> ImportFuncs {
        ImportFuncs { resolvers, stack }
    }
}

impl ResolverTarget for ImportFuncs {
    fn stack(&self) -> StackBasedMemory {
        self.stack.clone()
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
