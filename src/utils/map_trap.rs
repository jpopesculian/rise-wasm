use super::errors::{Error, RuntimeError};
use alloc::prelude::*;
use wasmi::Trap;

pub trait MapTrap {
    type Target;

    fn map_trap(self: Self) -> Self::Target;
}

impl<T, E> MapTrap for Result<T, E>
where
    E: Error + 'static,
{
    type Target = Result<T, Trap>;

    fn map_trap(self) -> Result<T, Trap> {
        self.map_err(|error| RuntimeError::new(Box::new(error)).into())
    }
}

impl<T> MapTrap for Option<T> {
    type Target = Result<T, Trap>;

    fn map_trap(self) -> Result<T, Trap> {
        self.ok_or(RuntimeError::new("Value not found").into())
    }
}
