use super::resolver::ResolverTarget;
use crate::memory::{
    AllocatorRef, Array, DynLittleEndianConvert, MemoryVal, MemoryWrapper, StorageVal, TableStorage,
};
use crate::utils::{CollectResult, ErrInto, MapTrap};
use alloc::prelude::*;
use byteorder::{ByteOrder, LittleEndian};
use core::convert::TryFrom;
use wasmi::{FromRuntimeValue, HostError, RuntimeArgs, RuntimeValue, Trap};

pub struct ResolverUtils<'a> {
    allocator: AllocatorRef,
    memory: MemoryWrapper,
    table: TableStorage,
    args: RuntimeArgs<'a>,
}

impl<'a> ResolverUtils<'a> {
    pub fn new<T: ResolverTarget>(target: &T, args: RuntimeArgs<'a>) -> ResolverUtils<'a> {
        ResolverUtils {
            allocator: target.allocator(),
            memory: target.memory(),
            table: target.table(),
            args,
        }
    }

    pub fn arg<T: FromRuntimeValue>(&self, idx: usize) -> Result<T, Trap> {
        self.args.nth_checked::<T>(idx)
    }

    pub fn mem_arg<T: DynLittleEndianConvert>(&self, idx: usize) -> Result<T, Trap> {
        self.memory.get_dyn_value::<T>(self.arg(idx)?).map_trap()
    }

    pub fn multi_mem_arg<T: DynLittleEndianConvert>(&self, idx: usize) -> Result<Vec<T>, Trap> {
        self.mem_arg::<Array>(idx)?
            .chunks()
            .iter()
            .map(|bytes| LittleEndian::read_u32(bytes))
            .map(|ptr| self.memory.get_dyn_value::<T>(ptr))
            .collect_result()
            .err_into()
    }

    pub fn table_arg<T>(&self, idx: usize) -> Result<T, Trap>
    where
        T: TryFrom<StorageVal>,
        T::Error: HostError,
    {
        let table_idx = self.arg(idx)?;
        let val = T::try_from(self.table.get(&table_idx).map_trap()?)?;
        Ok(val)
    }

    pub fn send<T: MemoryVal + DynLittleEndianConvert>(
        &self,
        val: T,
    ) -> Result<RuntimePointer, Trap> {
        let dest = self.allocator.clone().allocate(val.written_size() as u32)?;
        let _ = self.memory.set_dyn_value(dest, val)?;
        Ok(dest.into())
    }

    pub fn save<T: Into<StorageVal>>(
        &self,
        key: u32,
        val: T,
    ) -> Result<Option<RuntimeValue>, Trap> {
        self.table.insert(key, val.into()).map(|_| None).map_trap()
    }
}

pub struct RuntimePointer(u32);

impl RuntimePointer {
    pub fn new(val: u32) -> RuntimePointer {
        RuntimePointer(val)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl From<u32> for RuntimePointer {
    fn from(val: u32) -> RuntimePointer {
        RuntimePointer(val)
    }
}

impl From<RuntimePointer> for RuntimeValue {
    fn from(val: RuntimePointer) -> RuntimeValue {
        RuntimeValue::I32(val.get() as i32)
    }
}

pub struct RuntimeBool(bool);

impl RuntimeBool {
    pub fn new(val: bool) -> RuntimeBool {
        RuntimeBool(val)
    }

    pub fn get(&self) -> bool {
        self.0
    }
}

impl From<bool> for RuntimeBool {
    fn from(val: bool) -> RuntimeBool {
        RuntimeBool::new(val)
    }
}

impl From<RuntimeBool> for RuntimeValue {
    fn from(val: RuntimeBool) -> RuntimeValue {
        RuntimeValue::I32(if val.get() { 1 } else { 0 })
    }
}
