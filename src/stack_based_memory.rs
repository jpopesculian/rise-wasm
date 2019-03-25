use alloc::prelude::*;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Error;
use wasmi::memory_units::Pages;
use wasmi::{MemoryInstance, MemoryRef};

// @TODO: implement max size

#[derive(Debug, Clone)]
pub struct StackBasedMemory {
    memory: MemoryRef,
    values: Rc<RefCell<Vec<Vec<u8>>>>,
}

impl StackBasedMemory {
    pub fn new() -> StackBasedMemory {
        StackBasedMemory {
            memory: StackBasedMemory::build_memory(),
            values: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn build_memory() -> MemoryRef {
        MemoryInstance::alloc(Pages(1), Some(Pages(1))).expect("Memory could not be initialized")
    }

    pub fn memory(&self) -> MemoryRef {
        self.memory.clone()
    }

    pub fn push(&self, val: Vec<u8>) -> Result<(), Error> {
        self.values.borrow_mut().push(val.clone());
        Ok(())
    }

    pub fn pop(&self) -> Option<Vec<u8>> {
        self.values.borrow_mut().pop()
    }
}
