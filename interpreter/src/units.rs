use crate::MAX_DATABASE_BLKS;

//TODO This feels like a lot of boiler plate.
#[derive(Clone, Copy, Debug)]
pub struct DatabaseSize {
    inner: u32,
}

impl DatabaseSize {
    fn err() -> String {
        format!(
            "Database size must be from 100 to {} blocks",
            MAX_DATABASE_BLKS
        )
    }
    pub fn parse<'a>(val: &'a str) -> Result<Self, String> {
        let val = val.parse::<u32>().map_err(|_| Self::err())?;
        val.try_into()
    }
    pub fn inner(&self) -> u32 {
        self.inner
    }
}
impl TryFrom<u32> for DatabaseSize {
    type Error = String;
    fn try_from(val: u32) -> Result<Self, String> {
        if (100..MAX_DATABASE_BLKS).contains(&val) {
            //NOTE I am not sure why we need to set the lower bits
            //But it is done in the c code
            Ok(Self { inner: val | 7 })
        } else {
            Err(Self::err())
        }
    }
}
