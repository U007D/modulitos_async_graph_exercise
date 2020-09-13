use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorString(pub(crate) String);

impl Display for ErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ErrorString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::error::Error for ErrorString {}
