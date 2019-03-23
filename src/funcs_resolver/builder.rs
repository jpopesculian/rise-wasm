use super::{FuncResolver, ResolverTarget};
use alloc::prelude::*;
use hashbrown::HashMap;
use wasmi::{RuntimeArgs, RuntimeValue, Trap};

pub struct FuncsResolverBuilder<T> {
    indexes: HashMap<String, usize>,
    resolvers: Vec<Box<dyn FuncResolver<T>>>,
}

impl<T: ResolverTarget> FuncsResolverBuilder<T> {
    pub fn new() -> FuncsResolverBuilder<T> {
        FuncsResolverBuilder {
            indexes: HashMap::new(),
            resolvers: Vec::new(),
        }
    }

    pub fn push(
        mut self,
        name: &str,
        resolver: Box<dyn FuncResolver<T>>,
    ) -> FuncsResolverBuilder<T> {
        let index = self.resolvers.len();
        let _ = self.indexes.insert(name.into(), index);
        self.resolvers.push(resolver);
        self
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.indexes.get(name).cloned()
    }

    pub fn get_resolver(&self, index: usize) -> Option<&Box<dyn FuncResolver<T>>> {
        self.resolvers.get(index)
    }

    pub fn resolve(&self, name: &str) -> (Option<usize>, Option<&Box<dyn FuncResolver<T>>>) {
        match self.get_index(name) {
            Some(index) => (Some(index), self.get_resolver(index)),
            None => (None, None),
        }
    }
    pub fn run(
        &self,
        target: &mut T,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        self.get_resolver(index)
            .expect("Index should resolve to a function resolver")
            .run(target, args)
    }

    pub fn gas(&self, index: usize) -> u64 {
        self.get_resolver(index)
            .expect("Index should resolve to a function resolver")
            .gas()
    }
}

pub trait FuncResolverBuild<T> {
    fn build() -> Box<dyn FuncResolver<T>>;
}
