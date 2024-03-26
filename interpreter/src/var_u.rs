use crate::VAR_U;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct AlphaVAR_U {
    inner: VAR_U,
}

impl AlphaVAR_U {
    #[must_use]
    pub fn inner(&self) -> &VAR_U {
        &self.inner
    }
    /// # Errors
    ///
    /// Will return `Err` if the source string is not purly alpha chars or is too long.
    /// permission to read it.
    pub fn parse(val: &str) -> Result<Self, String> {
        val.try_into()
    }
}
impl TryFrom<&str> for AlphaVAR_U {
    type Error = String;
    fn try_from(val: &str) -> Result<Self, String> {
        match val.try_into() {
            Ok(val) => Ok(Self { inner: val }),
            Err(err) => Err(format!("{err} and only contain alpha characters")),
        }
    }
}
