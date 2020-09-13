mod ids;
mod node;
mod state;
#[cfg(test)]
mod unit_tests;

use crate::{error::client::Result, ports::Client};
use async_std::task;
use futures::executor::block_on;
use ids::Ids;
use node::{Node, NodeId};
use state::State;

#[derive(Debug)]
pub struct NodeTraverser<C: Client + Send> {
    client:    C,
    known_ids: Ids,
}

impl<C> NodeTraverser<C>
where
    C: Client + Send + Sync,
{
    pub fn new(client: C) -> Self {
        Self {
            client,
            known_ids: Ids::new(),
        }
    }

    pub async fn get<S>(&self, tgt: S) -> Result<Node>
    where
        S: AsRef<str> + Send + Sync, {
        let node = serde_json::from_str::<Node>(&self.client.get(tgt.as_ref()).await?)?;
        node.children().iter().for_each(|&node_id| {
            block_on(self.known_ids.push_unique(node_id));
        });
        Ok(node)
    }
}
