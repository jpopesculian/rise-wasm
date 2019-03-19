use alloc::string::String;
use wasmi::{
    Error, GlobalDescriptor, GlobalInstance, GlobalRef, ModuleImportResolver, RuntimeValue,
};

pub struct ImportGlobalsResolver {
    start_index: i32,
}

impl<'a> ImportGlobalsResolver {
    pub fn new(start_index: i32) -> ImportGlobalsResolver {
        ImportGlobalsResolver { start_index }
    }
}

impl<'a> ModuleImportResolver for ImportGlobalsResolver {
    fn resolve_global(
        &self,
        field_name: &str,
        _descriptor: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        let global_ref = match field_name {
            "start_index" => GlobalInstance::alloc(RuntimeValue::I32(self.start_index), false),
            _ => {
                return Err(Error::Function(String::from(
                    "host module doesn't export global with name",
                )));
            }
        };
        Ok(global_ref)
    }
}
