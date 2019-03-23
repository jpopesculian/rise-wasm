use alloc::string::String;
use wasmi::{Error, MemoryDescriptor, MemoryRef, ModuleImportResolver};

pub struct ImportMemoryResolver {
    pub memory: MemoryRef,
}

impl<'a> ImportMemoryResolver {
    pub fn new(memory: MemoryRef) -> ImportMemoryResolver {
        ImportMemoryResolver { memory }
    }
}

impl<'a> ModuleImportResolver for ImportMemoryResolver {
    fn resolve_memory(
        &self,
        field_name: &str,
        _descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        let mem_ref = match field_name {
            "default" => self.memory.clone(),
            _ => {
                return Err(Error::Function(String::from(
                    "host module doesn't export function with name",
                )));
            }
        };
        Ok(mem_ref)
    }
}
