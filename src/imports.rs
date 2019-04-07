use crate::funcs::{FuncsResolverBuilder, ResolverTarget};
use crate::globals::GlobalsRef;
use crate::memory::{AllocatorRef, MemoryWrapper, TableStorage, UninitializedAllocator};
use alloc::rc::Rc;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor,
    MemoryRef, ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, Trap,
};

#[derive(Debug)]
pub struct ImportResolver {
    table: TableStorage,
    resolvers: Rc<FuncsResolverBuilder<ImportResolver>>,
    memory: MemoryWrapper,
    allocator: AllocatorRef,
    globals: GlobalsRef,
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
        table: TableStorage,
        memory: MemoryWrapper,
        globals: GlobalsRef,
    ) -> ImportResolver {
        ImportResolver {
            resolvers,
            table,
            memory,
            globals,
            allocator: UninitializedAllocator::new(),
        }
    }
}

impl ResolverTarget for ImportResolver {
    fn table(&self) -> TableStorage {
        self.table.clone()
    }

    fn memory(&self) -> MemoryWrapper {
        self.memory.clone()
    }

    fn allocator(&self) -> AllocatorRef {
        self.allocator.clone()
    }

    fn set_allocator(&mut self, allocator: AllocatorRef) {
        self.allocator = allocator
    }
}

impl ModuleImportResolver for ImportResolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        match self.resolvers.resolve(field_name) {
            (Some(index), Some(resolver)) => Ok(FuncInstance::alloc_host(
                resolver.signature(signature),
                index,
            )),
            _ => Err(Error::Function(format!(
                "Could not find resolver with name '{}'",
                field_name
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
                return Err(Error::Function(format!(
                    "Could not find memory with name '{}'",
                    field_name
                )));
            }
        };
        Ok(mem_ref)
    }

    fn resolve_global(
        &self,
        field_name: &str,
        descriptor: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        self.globals
            .get(field_name, descriptor)
            .ok_or(Error::Function(format!(
                "Could not find global with name '{}'",
                field_name
            )))
    }
}
