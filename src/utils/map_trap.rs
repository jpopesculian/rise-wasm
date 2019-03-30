use alloc::prelude::*;
use core::fmt;
use core::marker;
use wasmi::{HostError, Trap, TrapKind};

type Error = Box<dyn fmt::Debug + marker::Send + marker::Sync>;

#[derive(Debug)]
pub struct RuntimeError(Error);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {:?}", self.0)
    }
}

impl HostError for RuntimeError {}

pub trait MapTrap {
    type Target;

    fn map_trap(self: Self) -> Self::Target;
}

impl<T, E> MapTrap for Result<T, E>
where
    E: fmt::Debug + marker::Send + marker::Sync + 'static,
{
    type Target = Result<T, Trap>;

    fn map_trap(self) -> Result<T, Trap> {
        self.map_err(|error| Trap::new(TrapKind::Host(Box::new(RuntimeError(Box::new(error))))))
    }
}

impl<T> MapTrap for Option<T> {
    type Target = Result<T, Trap>;

    fn map_trap(self) -> Result<T, Trap> {
        self.ok_or(Trap::new(TrapKind::Host(Box::new(RuntimeError(Box::new(
            "Value not found",
        ))))))
    }
}
