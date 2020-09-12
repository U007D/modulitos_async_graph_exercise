use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::empty_enum)] // remove once `Error` contains non-conditionally compiled variant
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[cfg(test)]
    #[error("Not found: {}", 0)]
    NotFound(String),
}
