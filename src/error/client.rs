use crate::consts::msg;
use super::error_string::ErrorString;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[cfg(test)]
    #[error("{}: {}", msg::ERR_NOT_FOUND, 0)]
    NotFound(ErrorString),
    #[error("{}: {}", msg::ERR_JSON_DESERIALIZATION, 0)]
    SerdeJsonError(ErrorString),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(ErrorString(err.to_string()))
    }
}
