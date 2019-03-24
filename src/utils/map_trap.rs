use wasmi::{Trap, TrapKind};

pub trait MapTrap {
    type Target;

    fn map_trap(self: Self, kind: TrapKind) -> Self::Target;
}

impl<T, E> MapTrap for Result<T, E> {
    type Target = Result<T, Trap>;

    fn map_trap(self, kind: TrapKind) -> Result<T, Trap> {
        self.map_err(|_| Trap::new(kind))
    }
}

impl<T> MapTrap for Option<T> {
    type Target = Result<T, Trap>;

    fn map_trap(self, kind: TrapKind) -> Result<T, Trap> {
        self.ok_or(Trap::new(kind))
    }
}
