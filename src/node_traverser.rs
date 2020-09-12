use crate::{error::Result, ports::Client};

#[cfg(test)]
mod unit_tests;

#[derive(Debug)]
pub struct NodeTraverser<C: Client> {
    client: C,
}

impl<C: Client> NodeTraverser<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn get<S: AsRef<str>>(&self, tgt: S) -> Result<String> {
        self.client.get(tgt.as_ref())
    }
}
