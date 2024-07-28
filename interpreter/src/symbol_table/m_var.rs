use super::var_u::VarU;
use crate::key::Key;
use ffi::u_char;

#[derive(Clone)]
pub struct MVar {
    pub name: VarU,
    volset: u_char,
    uci: u_char,
    pub key: Key,
}

impl std::fmt::Debug for MVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO implement a more complete implementation.
        //Currently this is just enough to start fuzz testing.
        let name = String::from_utf8(self.name.0.as_array().into()).unwrap();

        let mut builder = f.debug_struct("MVar");
        builder
            .field("name", &name)
            .field("key", &self.key)
            .field("volume set", &self.volset)
            .field("uci", &self.uci)
            .finish()
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod helpers {

    use super::{MVar, VarU};
    use crate::{key::Key, symbol_table::var_u::helpers::var_u, value::Value};
    use arbitrary::Arbitrary;
    use ffi::VAR_U;

    #[must_use]
    pub fn var_m(name: &str, values: &[&str]) -> MVar {
        let values = values
            .iter()
            .map(|x| Value::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        let key = Key::new(&values).unwrap();

        MVar {
            name: var_u(name),
            volset: Default::default(),
            uci: Default::default(),
            key,
        }
    }

    impl<'a> Arbitrary<'a> for MVar {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let name: [u8; 32] = u.arbitrary()?;
            if name.is_ascii() && name.contains(&0) {
                Ok(MVar {
                    name: VarU(VAR_U { var_cu: name }),
                    volset: 0,
                    uci: 0,
                    //TODO implement arbitrary for key.
                    key: Key::empty(),
                })
            } else {
                Err(arbitrary::Error::IncorrectFormat)
            }
        }
    }
}
