use super::funcs_resolver::{FuncsResolverBuilder, ResolverTarget};
use alloc::prelude::*;
use alloc::rc::Rc;
use core::fmt;
use wasmi::{isa::Instruction, Error, HostError, Middleware, MiddlewareEvent};

const DEFAULT_MAX: u64 = 100;

type GasForIndexFn = fn(usize) -> Option<u64>;

#[derive(Debug, Clone)]
pub struct GasMiddleware<T: ResolverTarget> {
    current_gas: u64,
    default_instruction_gas: u64,
    max_gas: u64,
    resolvers: Rc<FuncsResolverBuilder<T>>,
}

#[derive(Debug)]
pub struct GasMiddlewareError {}

impl fmt::Display for GasMiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of gas!")
    }
}

impl HostError for GasMiddlewareError {}

impl<T: ResolverTarget> GasMiddleware<T> {
    pub fn new(resolvers: Rc<FuncsResolverBuilder<T>>) -> GasMiddleware<T> {
        GasMiddleware {
            current_gas: 0,
            default_instruction_gas: 1,
            max_gas: DEFAULT_MAX,
            resolvers,
        }
    }

    fn check_gas(&mut self, instruction: &Instruction) -> Result<(), GasMiddlewareError> {
        self.current_gas += self.gas_for_instruction(instruction);
        if self.current_gas >= self.max_gas {
            Err(GasMiddlewareError {})
        } else {
            Ok(())
        }
    }

    fn gas_for_instruction(&self, instruction: &Instruction) -> u64 {
        match instruction {
            Instruction::Call(index) => {
                if let Some(gas) = self.resolvers.gas(*index as usize) {
                    gas
                } else {
                    self.default_instruction_gas
                }
            }
            _ => self.default_instruction_gas,
        }
    }
}

impl<T> Middleware for GasMiddleware<T>
where
    T: fmt::Debug + ResolverTarget,
{
    fn handle(&mut self, event: MiddlewareEvent) -> Result<(), Error> {
        match event {
            MiddlewareEvent::Instruction(instruction) => self
                .check_gas(instruction)
                .map_err(|err| Error::Host(Box::new(err))),
        }
    }
}
