use crate::{MAX_STR_LEN, Value};

impl TryFrom<&str> for Value {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() <= MAX_STR_LEN {
            Ok(Self(Vec::from(value.as_bytes())))
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() <= MAX_STR_LEN {
            Ok(Self(Vec::from(value)))
        } else {
            Err(())
        }
    }
}

impl std::str::FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
