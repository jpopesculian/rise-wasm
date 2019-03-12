use alloc::string::String;
use wasm_bindgen::prelude::*;
use wasmi::{
    Externals, RuntimeValue, RuntimeArgs, Error, ModuleImportResolver,
    FuncRef, ValueType, Signature, FuncInstance, Trap,
};
use crate::import_memory::ImportMemoryResolver;


#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn minus(x: i32, y: i32) -> i32;
    fn look(mem: &[u8]);
    fn hash160(bytes: &[u8]) -> JsValue;
    fn compare(bytes1: &[u8], bytes2: &[u8]) -> i32;
    fn verify_sig(sig: &[u8], pubkey: &[u8]) -> i32;
}

pub struct ImportFuncs<'a> {
    mem: &'a ImportMemoryResolver
}

const ADD_FUNC_INDEX: usize = 0;
const MINUS_FUNC_INDEX: usize = 1;
const LOOK_FUNC_INDEX: usize = 2;
const HASH160_FUNC_INDEX: usize = 3;
const COMPARE_FUNC_INDEX: usize = 4;
const VERIFY_SIG_FUNC_INDEX: usize = 5;

impl<'a> Externals for ImportFuncs<'a> {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            ADD_FUNC_INDEX => {
                let a: u32 = args.nth_checked(0)?;
                let b: u32 = args.nth_checked(1)?;
                let result = a + b;
                Ok(Some(RuntimeValue::I32(result as i32)))
            }
            MINUS_FUNC_INDEX => {
                let a: i32 = args.nth_checked(0)?;
                let b: i32 = args.nth_checked(1)?;
                let result = minus(a, b);
                Ok(Some(RuntimeValue::I32(result as i32)))
            }
            LOOK_FUNC_INDEX => {
                let offset: u32 = args.nth_checked(0)?;
                let len: u32 = args.nth_checked(1)?;
                look(
                    &self.mem.memory.get(offset, len as usize)
                        .expect("could not get memory")
                );
                Ok(None)
            },
            HASH160_FUNC_INDEX => {
                let input: u32 = args.nth_checked(0)?;
                let output: u32 = args.nth_checked(1)?;
                let result = hash160(
                    &self.mem.memory.get(input, 66)
                        .expect("could not get memory")
                ).as_string().expect("Didn't receive string");
                self.mem.memory.set(output, result.as_bytes())
                    .expect("Couldn't write to memory");
                Ok(None)
            },
            COMPARE_FUNC_INDEX => {
                let start1: u32 = args.nth_checked(0)?;
                let start2: u32 = args.nth_checked(1)?;
                let len: u32 = args.nth_checked(2)?;

                let bytes1 = &self.mem.memory.get(start1, len as usize)
                        .expect("could not get memory");
                let bytes2 = &self.mem.memory.get(start2, len as usize)
                        .expect("could not get memory");

                Ok(Some(RuntimeValue::I32(compare(bytes1, bytes2))))
            },
            VERIFY_SIG_FUNC_INDEX => {
                let start_sig: u32 = args.nth_checked(0)?;
                let start_pubkey: u32 = args.nth_checked(1)?;

                let sig = &self.mem.memory.get(start_sig, 128)
                        .expect("could not get memory");
                let pubkey = &self.mem.memory.get(start_pubkey, 66)
                        .expect("could not get memory");

                Ok(Some(RuntimeValue::I32(verify_sig(sig, pubkey))))
            },
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}

impl<'a> ImportFuncs<'a> {
    pub fn new(mem: &'a ImportMemoryResolver) -> ImportFuncs<'a> {
        return ImportFuncs { mem };
    }
}

pub struct ImportFuncsResolver;

impl<'a> ImportFuncsResolver {
    pub fn new() -> ImportFuncsResolver {
        return ImportFuncsResolver {}
    }
}

impl<'a> ModuleImportResolver for ImportFuncsResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature
    ) -> Result<FuncRef, Error> {
        let func_ref = match field_name {
            "add" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
                ADD_FUNC_INDEX
            ),
            "minus" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
                MINUS_FUNC_INDEX
            ),
            "look" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                LOOK_FUNC_INDEX
            ),
            "hash160" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], None),
                HASH160_FUNC_INDEX
            ),
            "compare" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
                COMPARE_FUNC_INDEX
            ),
            "verify_sig" => FuncInstance::alloc_host(
                Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
                VERIFY_SIG_FUNC_INDEX
            ),
            _ => {
                return Err(Error::Function(
                    String::from("host module doesn't export function with name")
                ));
            }
        };
        Ok(func_ref)
    }
}
