use alloc::prelude::*;
use core::fmt;
use core::iter;
use core::ops::{Deref, DerefMut};
use core::result::Result as CoreResult;
use wasmi::{Error as WasmiError, HostError};

pub trait Error: fmt::Debug + fmt::Display + Send + Sync {}
impl<T: fmt::Debug + fmt::Display + Send + Sync> Error for T {}

pub type BoxedError = Box<dyn Error>;

pub struct Result<T>(CoreResult<T, RuntimeError>);

impl<T> Deref for Result<T> {
    type Target = CoreResult<T, RuntimeError>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Result<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct RuntimeError(BoxedError);

impl RuntimeError {
    pub fn new<T: Error + 'static>(error: T) -> RuntimeError {
        RuntimeError(Box::new(error))
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {:?}", self.0)
    }
}

impl HostError for RuntimeError {}

impl<T, E> From<CoreResult<T, E>> for Result<T>
where
    E: Error + 'static,
{
    fn from(err: CoreResult<T, E>) -> Result<T> {
        Result(err.map_err(|err| RuntimeError::new(err)))
    }
}

impl From<WasmiError> for RuntimeError {
    fn from(err: WasmiError) -> RuntimeError {
        RuntimeError::new(err)
    }
}

pub trait ErrInto<T, E> {
    fn err_into(self: Self) -> CoreResult<T, E>;
}

impl<T, E1, E2> ErrInto<T, E2> for CoreResult<T, E1>
where
    E1: Into<E2>,
{
    fn err_into(self) -> CoreResult<T, E2> {
        self.map_err(|error| error.into())
    }
}

pub trait CollectResult<T, E> {
    fn collect_result(self: Self) -> CoreResult<Vec<T>, E>;
}

impl<T, E, I> CollectResult<T, E> for I
where
    E: Error,
    I: iter::Iterator<Item = CoreResult<T, E>>,
{
    fn collect_result(self: Self) -> CoreResult<Vec<T>, E> {
        let mut result: Vec<T> = Vec::new();
        for item in self {
            match item {
                Ok(item) => result.push(item),
                Err(err) => return Err(err),
            };
        }
        Ok(result)
    }
}

// impl<T> From<Result<T, Error>> for Result<T, RuntimeError> {
//     fn from(error: Result<T, Error>) -> Result<T, RuntimeError> {
//         RuntimeError::new(error)
//     }
// }
