use alloc::string::String;
use wasmi::{Error, GlobalDescriptor, GlobalRef, ModuleImportResolver};

pub struct ImportGlobalsResolver {}

impl<'a> ImportGlobalsResolver {
    pub fn new() -> ImportGlobalsResolver {
        ImportGlobalsResolver {}
    }
}

impl<'a> ModuleImportResolver for ImportGlobalsResolver {
    fn resolve_global(
        &self,
        field_name: &str,
        _descriptor: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        let global_ref = match field_name {
            _ => {
                return Err(Error::Function(String::from(
                    "host module doesn't export global with name",
                )));
            }
        };
        Ok(global_ref)
    }
}
