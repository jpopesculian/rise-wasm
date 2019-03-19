use alloc::string::String;
use wasmi::memory_units::Pages;
use wasmi::{Error, MemoryDescriptor, MemoryInstance, MemoryRef, ModuleImportResolver};

pub struct ImportMemoryResolver {
    pub memory: MemoryRef,
}

impl<'a> ImportMemoryResolver {
    pub fn new(args: &[u8]) -> ImportMemoryResolver {
        ImportMemoryResolver {
            memory: ImportMemoryResolver::build_memory(args),
        }
    }

    fn build_memory(args: &[u8]) -> MemoryRef {
        let mem_ref = MemoryInstance::alloc(Pages(1), Some(Pages(1)))
            .expect("Memory could not be initialized");
        mem_ref.set(0, args).expect("Couldn't set memory");
        mem_ref
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
