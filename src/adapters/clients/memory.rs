use crate::consts::SERVER_LATENCY_MS_NODE_B;
use crate::error::client::{Error, Result};
use crate::ports::Client;
use async_std::task;
use async_trait::async_trait;
use std::time::Duration;

#[derive(Debug)]
pub struct MemoryClient {}

impl MemoryClient {
    pub const fn new() -> Self {
        Self {}
    }

    async fn inner_get(tgt: &str) -> Result<String> {
        match tgt {
            "a" => Ok(r#"
                {
                   "children": [ "b", "e" ],
                   "reward": 1
                }"#
            .into()),
            "b" => {
                task::sleep(Duration::from_millis(SERVER_LATENCY_MS_NODE_B)).await;
                Ok(r#"
                    {
                        "children": [ "c" ],
                        "reward": 2
                    }"#
                .into())
            }
            "e" => Ok(r#"
                {
                    "children": [ "f" ],
                    "reward": 4
                }"#
            .into()),
            _ => Err(Error::NotFound(tgt.to_string().into())),
        }
    }
}

// unexpected `#[async_trait]` interaction with trait bounds
#[allow(clippy::type_repetition_in_bounds)]
#[async_trait]
impl Client for MemoryClient {
    async fn get<S>(&self, tgt: S) -> Result<String>
    where
        S: AsRef<str> + Send + Sync,
    {
        Ok(Self::inner_get(tgt.as_ref()).await?)
    }
}
