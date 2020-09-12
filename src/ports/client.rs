use crate::error::Result;

pub trait Client {
    fn get<P: AsRef<str>>(&self, tgt: P) -> Result<String>;
}
˚