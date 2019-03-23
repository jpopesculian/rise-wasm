use super::{FuncResolver, FuncResolverBuild, ResolverTarget};
use crate::log;
use alloc::prelude::*;
use core::ops::Deref;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap, TrapKind, ValueType};

#[wasm_bindgen(module = "../js/imports")]
extern "C" {
    fn hash160(bytes: &[u8]) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct Buffer {
    data: Vec<u8>,
}

impl Deref for Buffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct Hash160Resolver;

impl<T: ResolverTarget> FuncResolver<T> for Hash160Resolver {
    fn signature(&self, _: &Signature) -> Signature {
        Signature::new(&[][..], None)
    }

    fn run(&self, target: &mut T, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        let stack = target.stack();
        let value = stack
            .pop()
            .ok_or(Trap::new(TrapKind::MemoryAccessOutOfBounds))?;
        let hash: Buffer = hash160(&value)
            .into_serde()
            .map_err(|err| Trap::new(TrapKind::Unreachable))?;
        stack
            .push(hash.to_vec())
            .map_err(|_| Trap::new(TrapKind::Unreachable))?;
        Ok(None)
    }

    fn gas(&self) -> u64 {
        10
    }
}

impl<T: ResolverTarget> FuncResolverBuild<T> for Hash160Resolver {
    fn build() -> Box<dyn FuncResolver<T>> {
        Box::new(Hash160Resolver {})
    }
}
