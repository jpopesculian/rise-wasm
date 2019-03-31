use super::funcs_resolver::{FuncsResolverBuilder, ResolverTarget};
use super::StackBasedMemory;
use super::MemoryWrapper;
use alloc::prelude::*;
use alloc::rc::Rc;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue,
    Signature, Trap, MemoryRef, MemoryDescriptor
};

#[derive(Debug)]
pub struct ImportResolver {
    stack: StackBasedMemory,
    resolvers: Rc<FuncsResolverBuilder<ImportResolver>>,
    memory: MemoryWrapper
}

impl Externals for ImportResolver {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        self.resolvers.clone().run(self, index, args)
    }
}

impl ImportResolver {
    pub fn new(
        resolvers: Rc<FuncsResolverBuilder<ImportResolver>>,
        stack: StackBasedMemory,
        memory: MemoryWrapper,
    ) -> ImportResolver {
        ImportResolver { resolvers, stack, memory }
    }
}

impl ResolverTarget for ImportResolver {
    fn stack(&self) -> StackBasedMemory {
        self.stack.clone()
    }

    fn memory(&self) -> MemoryWrapper {
        self.memory.clone()
    }
}

impl ModuleImportResolver for ImportResolver {
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

    fn resolve_memory(
        &self,
        field_name: &str,
        _descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        let mem_ref = match field_name {
            "memory" => self.memory.raw(),
            _ => {
                return Err(Error::Function(String::from(
                    "host module doesn't export function with name",
                )));
            }
        };
        Ok(mem_ref)
    }
}
