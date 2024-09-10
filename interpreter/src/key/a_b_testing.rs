use crate::value::Value;
use arbitrary::Arbitrary;
use ffi::{
    symbol_table::{build_key, extract_key, string_key},
    ERRMLAST, ERRZ1, ERRZ5,
};

impl<'a> Arbitrary<'a> for NonNullableKey {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let keys: Vec<_> = u.arbitrary()?;
        match Self::new(&keys) {
            Ok(key) => Ok(key),
            Err(_) => Err(arbitrary::Error::IncorrectFormat),
        }
    }
}

impl<'a> Arbitrary<'a> for NullableKey {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let keys: Vec<_> = u.arbitrary()?;
        match Self::new(&keys) {
            Ok(key) => Ok(key),
            Err(_) => Err(arbitrary::Error::IncorrectFormat),
        }
    }
}

use super::{Error, NonNullableKey, NullableKey};

//TODO all of these should be revamped to work on arrays of keys.
#[cfg_attr(test, mutants::skip)]
pub fn build(value: &Value) {
    let key = super::NullableKey::new([value]);
    let result = key.map(|x| x.0).map_err(|x| match x {
        Error::SubscriptToLarge => -((ERRZ1 + ERRMLAST) as i16),
        Error::SubKeyContainsNull => -((ERRZ5 + ERRMLAST) as i16),
        _ => unreachable!(),
    });
    assert_eq!(result, build_key(&value.clone().into_cstring()));
}

//TODO push key creation up to calling code.
#[cfg_attr(test, mutants::skip)]
pub fn extract(string: &Value) -> Result<(), Error> {
    let key = super::NullableKey::new([string])?;
    assert_eq!(key.key_extract(false), extract_key(&key.0).unwrap());
    Ok(())
}

//TODO push key creation up to calling code.
#[cfg_attr(test, mutants::skip)]
pub fn string(keys: &[Value]) -> Result<(), Error> {
    let key_list = NullableKey::new(keys)?;
    assert_eq!(key_list.string_key(), string_key(&key_list.0[..], i32::MAX));
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("")]
    #[case(".")]
    #[case("1.")]
    #[case("test string")]
    #[case("0")]
    #[case("10")]
    #[case("10E")]
    #[case("10.4")]
    #[case(".4")]
    #[case("10.4E")]
    #[case("10.0")]
    #[case("010")]
    #[case("10.5.")]
    #[case("-")]
    #[case("-.")]
    #[case("-10")]
    #[case("-10E")]
    #[case("-10.4")]
    #[case("-.4")]
    #[case("-4.")]
    #[case("-10.4E")]
    #[case("-10.0")]
    #[case("-010")]
    #[case("-10.5.")]
    fn build_a_b_test_cases(#[case] input: Value) {
        build(&input);
        extract(&input).expect("all of the test strings should produce valid keys");
    }

    #[rstest]
    #[case(&["only"])]
    #[case(&[""])]
    #[case(&["\""])]
    #[case(&["9"])]
    #[case(&["-9"])]
    #[case(&["-9.0"])]
    #[case(&["f","s"])]
    #[case(&["","s","9","-9"])]
    fn key_extract_string(#[case] raw_keys: &[&str]) {
        let keys = raw_keys
            .iter()
            .map(|x| (*x).try_into().unwrap())
            .collect::<Vec<_>>();
        matches!(string(&keys), Ok(()));
    }
}
