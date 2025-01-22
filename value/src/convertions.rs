use crate::{MAX_STR_LEN, Value};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum CreationError {
    #[error("Max string length is {MAX_STR_LEN}")]
    ExceededMaxStringLen,
}

impl TryFrom<&str> for Value {
    type Error = CreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() <= MAX_STR_LEN {
            Ok(Self(Vec::from(value.as_bytes())))
        } else {
            Err(CreationError::ExceededMaxStringLen)
        }
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = CreationError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() <= MAX_STR_LEN {
            Ok(Self(Vec::from(value)))
        } else {
            Err(CreationError::ExceededMaxStringLen)
        }
    }
}

impl std::str::FromStr for Value {
    type Err = CreationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
