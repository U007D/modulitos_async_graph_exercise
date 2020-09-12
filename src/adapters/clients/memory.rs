use crate::{
    error::client::{Error as ClientError, Result as ClientResult},
    ports::Client,
    Result,
};

#[derive(Debug)]
pub struct MemoryClient {}

impl MemoryClient {
    pub const fn new() -> Self {
        Self {}
    }

    #[allow(clippy::unused_self)]
    fn inner_get(&self, tgt: &str) -> ClientResult<String> {
        match tgt {
            "a" => Ok(r#"
                {
                   "children": [ "b", "e" ],
                   "reward": 1
                }"#
            .into()),
            _ => Err(ClientError::NotFound(tgt.into())),
        }
    }
}

impl Client for MemoryClient {
    fn get<S: AsRef<str>>(&self, tgt: S) -> Result<String> {
        Ok(self.inner_get(tgt.as_ref())?)
    }
}
