#![allow(dead_code)]
use arbitrary::Arbitrary;
use derive_more::{AsMut, AsRef};
use ffi::CSTRING;
use ref_cast::RefCast;

mod internal;
use internal::ParsedKey;

//TODO find a better name for this.
//CString is already used by the std.
#[derive(RefCast, AsMut, AsRef)]
#[repr(transparent)]
#[derive(Clone)] //NOTE keep the Manual Debug implementation in sync
pub struct CArrayString(CSTRING);

impl CArrayString {
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.0.buf[..self.0.len as usize]
    }
    #[must_use]
    pub fn new(c_string: CSTRING) -> Self {
        Self(c_string)
    }
}
impl TryFrom<&[u8]> for CArrayString {
    //Currently will only error when the slice is to large
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() <= 65535 {
            let mut buf = [0; 65535];
            buf[..value.len()].clone_from_slice(value);
            Ok(CArrayString::new(ffi::CSTRING {
                len: value.len() as u16,
                buf,
            }))
        } else {
            Err(())
        }
    }
}

impl std::fmt::Debug for CArrayString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CArrayString")
            .field("content", &self.content())
            .field("content_as_utf8", &std::str::from_utf8(self.content()))
            .finish()
    }
}

impl PartialEq for CArrayString {
    fn eq(&self, other: &Self) -> bool {
        self.content() == other.content()
    }
}

impl<'a> Arbitrary<'a> for CArrayString {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(CSTRING {
            len: u16::arbitrary(u)?,
            buf: Arbitrary::arbitrary(u)?,
        }))
    }
}

/// Stores a list of keys.
/// This is a work in progress
///
/// NOTE this currently only supports one key.
pub struct List(Vec<u8>);
impl List {
    #[must_use]
    pub fn new() -> Self {
        Self(vec![0])
    }

    fn extend(&mut self, iter: impl std::iter::Iterator<Item = CArrayString>) -> Result<(), Error> {
        for key in iter {
            self.push(&key)?;
        }
        Ok(())
    }

    fn string_key(&self) -> Vec<u8> {
        let mut out_put = vec![b'('];
        let mut keys = self
            .iter()
            .map(|x| ParsedKey::from_key_ref(x).external_fmt(true))
            .peekable();
        let non_empty = keys.peek().is_some();

        for key in keys {
            out_put.extend(key);
            out_put.push(b',');
        }
        if non_empty {
            //change the last ',' to a ')'.
            *out_put.last_mut().unwrap() = b')';
        } else {
            out_put.push(b')');
        }
        out_put
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0[0] as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[cfg(any(test, feature = "fuzzing"))]
    pub fn raw_keys(&self) -> &[u8] {
        &self.0[1..]
    }

    //Note I should probably remove this at some point.
    //It currently assumes there is at least one key in storage.
    #[must_use]
    pub fn key_extract(&self, quote_strings: bool) -> Vec<u8> {
        ParsedKey::from_key_ref(self.iter().next().unwrap()).external_fmt(quote_strings)
    }

    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter { tail: &self.0[1..] }
    }
}

//This lint seems to be a false positive.
#[allow(clippy::into_iter_without_iter)]
impl<'a> IntoIterator for &'a List {
    type IntoIter = Iter<'a>;
    type Item = Ref<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Ref<'a>(&'a [u8]);
pub struct Iter<'a> {
    tail: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InputToLarge,
    ContainsNull,
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod a_b_testing {
    use std::ptr::{from_mut, from_ref};

    use ffi::{UTIL_Key_Build, UTIL_Key_Extract, ERRMLAST, ERRZ1, ERRZ5, MAX_STR_LEN};

    use super::{CArrayString, Error, List};

    //TODO all of these should be revamped to work on arrays of keys.
    pub fn build(string: &CArrayString) {
        let mut keys = super::List::new();
        let result = keys.push(string);
        let mut buffer = [0; MAX_STR_LEN as usize + 1];
        let len =
            unsafe { UTIL_Key_Build(from_ref(string.as_ref()).cast_mut(), buffer.as_mut_ptr()) };
        match result {
            Ok(()) => {
                assert_eq!(keys.raw_keys(), &buffer[..len as usize]);
                //The c code should never write past key's len, but this is good to check.
                assert!(buffer[len as usize..].iter().all(|x| *x == 0));
            }
            Err(Error::InputToLarge) => {
                assert_eq!(-len, (ERRZ1 + ERRMLAST) as i16);
            }
            Err(Error::ContainsNull) => {
                assert_eq!(-len, (ERRZ5 + ERRMLAST) as i16);
            }
        }
    }

    pub fn extract(string: &CArrayString) -> Result<(), Error> {
        let mut key_list = super::List::new();
        key_list.push(string)?;
        let mut output_buffer = [0; MAX_STR_LEN as usize + 1];
        let mut count = 0;

        //less then zero means there was a error building the key.
        let len = unsafe {
            UTIL_Key_Extract(
                key_list.raw_keys().as_ptr().cast_mut(),
                output_buffer.as_mut_ptr(),
                from_mut(&mut count),
            )
        };

        let output = key_list.key_extract(false);
        assert_eq!(output[..], output_buffer[..len as usize]);
        Ok(())
    }

    pub fn string_key(keys: &[CArrayString]) -> Result<(), Error> {
        let mut key_list = List::new();
        key_list.extend(keys.iter().cloned())?;
        let output = key_list.string_key();

        let mut output_buffer = [0; MAX_STR_LEN as usize + 1];

        //less then zero means there was a error building the key.
        let len = unsafe {
            ffi::UTIL_String_Key(
                key_list.0.as_mut_ptr(),
                output_buffer.as_mut_ptr(),
                keys.len() as i32,
            )
        };

        assert_eq!(&output, &output_buffer[..len as usize]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use a_b_testing::string_key;
    use ffi::MAX_SUB_LEN;
    use ffi::{CSTRING, MAX_STR_LEN};
    use internal::MAX_INT_SEGMENT_SIZE;
    use rstest::rstest;

    use super::*;

    impl From<&str> for CArrayString {
        fn from(value: &str) -> Self {
            let mut buffer = [0; MAX_STR_LEN as usize + 1];
            buffer[..value.len()].copy_from_slice(value.as_bytes());
            CArrayString(CSTRING {
                len: value.len().try_into().unwrap(),
                buf: buffer,
            })
        }
    }

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
    fn build_a_b_test_cases(#[case] input: &str) {
        super::a_b_testing::build(&input.into());
        super::a_b_testing::extract(&input.into())
            .expect("all of the test strings should produce valid keys");
    }

    #[test]
    fn max_key_size() {
        let mut keys = List::new();
        let result = keys.push(&"a".repeat(MAX_SUB_LEN as usize).as_str().into());
        assert_eq!(result, Ok(()));
    }
    #[test]
    fn key_that_is_to_large() {
        let mut keys = List::new();
        let result = keys.push(&"a".repeat(MAX_SUB_LEN as usize + 1).as_str().into());
        assert_eq!(result, Err(Error::InputToLarge));
    }

    #[test]
    fn error_if_string_contains_null() {
        let mut keys = List::new();
        let result = keys.push(&"a\0b".into());
        assert_eq!(result, Err(Error::ContainsNull));
    }

    #[test]
    fn build_key_int_to_large() {
        let src: CArrayString = "1".repeat(MAX_INT_SEGMENT_SIZE + 1).as_str().into();
        let mut keys = List::new();
        keys.push(&src).unwrap();
        assert!(matches!(
            ParsedKey::from_key_ref(Ref(keys.raw_keys())),
            ParsedKey::String(_)
        ));
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
        let keys = raw_keys.iter().map(|x| (*x).into()).collect::<Vec<_>>();
        matches!(string_key(&keys), Ok(()));
    }

    #[test]
    fn key_cmp() -> Result<(), Error> {
        //NOTE CStrings are really large so putting 4 in a array will over flow the unit tests
        //stack
        let keys = ["", "-9.9", "-9.8", "-9", "0", "9", "9,8"];
        //let keys = ["", "-9.9", "-9", "0", "9", "9.9", "string"].map(|x| x.into());
        for [a, b] in keys.array_windows() {
            let mut list = List::new();
            list.push(&dbg!((*a).into()))?;
            list.push(&dbg!((*b).into()))?;
            let mut iter = list.iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            assert!(a < b);
        }
        Ok(())
    }
}
