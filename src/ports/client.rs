use crate::error::client::Result;
use async_trait::async_trait;

// unexpected `#[async_trait]` interaction with trait bounds
#[allow(clippy::type_repetition_in_bounds)]
#[async_trait]
pub trait Client {
    async fn get<S>(&self, tgt: S) -> Result<String>
    where
        S: AsRef<str> + Send + Sync;
}
