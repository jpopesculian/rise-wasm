use alloc::prelude::*;
use alloc::rc::Rc;
use core::cell::RefCell;
use hashbrown::HashMap;
use wasmi::{GlobalDescriptor, GlobalInstance, GlobalRef, RuntimeValue};

#[derive(Debug, Clone)]
pub struct Globals {
    map: HashMap<String, GlobalRef>,
}

#[derive(Debug, Clone)]
pub struct GlobalsRef(Rc<RefCell<Globals>>);

impl Globals {
    pub fn default() -> GlobalsRef {
        GlobalsRef(Rc::new(RefCell::new(Globals {
            map: HashMap::new(),
        })))
    }

    pub fn insert<T: Into<RuntimeValue>>(&mut self, key: String, value: T) {
        let global = GlobalInstance::alloc(value.into(), false);
        self.map.insert(key, global);
    }

    pub fn get(&self, key: String, descriptor: &GlobalDescriptor) -> Option<GlobalRef> {
        if let Some(global) = self.map.get(&key) {
            if global.value_type() == descriptor.value_type()
                && global.is_mutable() == descriptor.is_mutable()
            {
                Some(global.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl GlobalsRef {
    pub fn with_global<T: Into<RuntimeValue>>(self, key: &str, value: T) -> GlobalsRef {
        self.0.borrow_mut().insert(key.into(), value);
        self
    }

    pub fn get(&self, key: &str, descriptor: &GlobalDescriptor) -> Option<GlobalRef> {
        self.0.borrow().get(key.into(), descriptor)
    }
}
