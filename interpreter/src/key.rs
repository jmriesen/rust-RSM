#![allow(dead_code)]
use arbitrary::Arbitrary;
use derive_more::{AsMut, AsRef};
use ffi::{CSTRING, MAX_SUB_LEN};
use ref_cast::RefCast;

//TODO I will eventually want to be able to write a string in place.
type Key = Vec<u8>;

static STRING_FLAG: u8 = 0b1000_0000;
static NON_NEGATIVE: u8 = 0b100_0000;

//TODO find a better name for this.
//CString is already used by the std.
#[derive(RefCast, AsMut, AsRef)]
#[repr(transparent)]
#[derive(Clone)] //NOTE keep the Manual Debug implementation in sync
pub struct CArrayString(CSTRING);

impl CArrayString {
    pub fn content(&self) -> &[u8] {
        &self.0.buf[..self.0.len as usize]
    }
}

impl std::fmt::Debug for CArrayString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CArrayString")
            .field("content", &self.content())
            .field("content_as_utf8", &std::str::from_utf8(&self.content()))
            .finish()
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
struct KeyList(Vec<u8>);
impl KeyList {
    fn new() -> Self {
        Self(vec![0])
    }

    /// Remove once refactor is compleate.
    fn from_raw_key(buff: &[u8]) -> Self {
        let mut temp = vec![0];
        temp.extend(buff);
        Self(temp)
    }

    fn extend(
        &mut self,
        iter: impl std::iter::Iterator<Item = CArrayString>,
    ) -> Result<(), KeyError> {
        for key in iter.map(|x| key_build(&x)).collect::<Result<Vec<_>, _>>()? {
            self.0.extend(key);
        }
        self.0[0] = (self.0.len() + 1) as u8;
        Ok(())
    }

    fn string_key(&self) -> Vec<u8> {
        dbg!(&self.0);
        let mut tmp = key_extract(&self.0[1..], true);
        tmp.insert(0, b'(');
        tmp.push(b')');
        dbg!(tmp)
    }

    fn len(&self) -> usize {
        self.0[0] as usize
    }

    fn raw_keys(&self) -> &[u8] {
        &self.0[1..]
    }

    fn push(&mut self, src: &CArrayString) -> Result<(), KeyError> {
        //TODO update the size
        let internal_key = KeyInternal::new(&src)?;
        let end_mark = match internal_key {
            KeyInternal::Null => {
                self.0.push(b'\0');
                None
            }
            KeyInternal::Zero => {
                self.0.push(NON_NEGATIVE);
                None
            }
            KeyInternal::Positive { int_part, dec_part } => {
                self.0.push(NON_NEGATIVE + int_part.len() as u8);
                self.0.extend(int_part);
                self.0.extend(dec_part);
                None
            }
            KeyInternal::Negative { int_part, dec_part } => {
                self.0.push(63 - int_part.len() as u8);
                //TODO figure out how this complement is supposed to work.
                self.0.extend(int_part);
                self.0.extend(dec_part);
                Some(255)
            }
            KeyInternal::String(contents) => {
                self.0.push(STRING_FLAG);
                self.0.extend(contents);
                None
            }
            KeyInternal::Bug => {
                //I think this case is a bug in the C code
                //For now I am replicating the behavior
                //Encoding as -0
                self.0.extend([63, 255]);
                return Ok(());
            }
        };
        //Adding end markers
        //NOTE Negatives use 255 as an end mark for some reason.
        //at some point when I understand 9's complement better I should se if I can remove this
        self.0.push(end_mark.unwrap_or(b'\0'));
        Ok(())
    }

    pub fn key_extract(&self, quote_strings: bool) -> Vec<u8> {
        match KeyInternal::from_slice(self.raw_keys()) {
            KeyInternal::Null => {
                if quote_strings {
                    vec![b'"', b'"']
                } else {
                    vec![]
                }
            }
            KeyInternal::Zero => vec![b'0'],
            KeyInternal::Positive { int_part, dec_part } => {
                let mut key = Vec::from(int_part);
                if !dec_part.is_empty() {
                    key.push(b'.');
                    key.extend(dec_part);
                }
                key
            }
            KeyInternal::Negative { int_part, dec_part } => {
                let mut key = vec![b'-'];
                key.extend(int_part.map(|x| b'9' + b'0' - x));
                let mut dec_part = dec_part.peekable();
                if dec_part.peek().is_some() {
                    key.push(b'.');
                    key.extend(dec_part.map(|x| b'9' + b'0' - x));
                }
                key
            }
            KeyInternal::String(string) => {
                let mut string = Vec::from(string);
                if quote_strings {
                    string.insert(0, b'"');
                    string.push(b'"');
                }
                string
            }
            KeyInternal::Bug => {
                vec![b'-']
            }
        }
    }

    unsafe fn as_raw(&mut self) -> *mut u8 {
        dbg!(&mut self.0).as_mut_ptr()
    }
}

#[derive(Debug, PartialEq)]
pub enum KeyError {
    InputToLarge,
    ContainsNull,
}

enum KeyInternal<'a> {
    Null,
    Zero,
    Positive {
        int_part: &'a [u8],
        dec_part: &'a [u8],
    },
    Negative {
        //lastly converting to nines complement
        //dyn so I can provide a no op if the source bytes are already in nine's complement
        int_part: Box<dyn ExactSizeIterator<Item = u8> + 'a>,
        dec_part: Box<dyn ExactSizeIterator<Item = u8> + 'a>,
    },
    ///NOTE if a number is to large it is encoded as a string
    String(&'a [u8]),
    //TODO file issue request upstream
    // it looks like the key "-." is not parsed properly
    Bug,
}

impl<'a> KeyInternal<'a> {
    fn new(src: &'a CArrayString) -> Result<Self, KeyError> {
        let contents = src.content();
        if contents.len() > MAX_SUB_LEN as usize {
            //consider reevaluating this.
            //the restrictions seems a bit arbitrary
            Err(KeyError::InputToLarge)
        } else if contents == &[] {
            Ok(Self::Null)
        } else if contents == &[b'0'] {
            Ok(Self::Zero)
        } else if contents == &[b'-', b'.'] {
            Ok(Self::Bug)
        } else {
            //attempt to parse as a number
            let negative = contents.starts_with(&[b'-']);
            let mut parts = if negative {
                &contents[1..]
            } else {
                &contents[..]
            }
            .split(|x| *x == b'.');

            let int_part = parts
                .next()
                .expect("empty string case should have already been handled");
            let dec_part = parts.next();
            let trailing_dot = dec_part == Some(&[]) && !negative; // the anding with !negative seems like a but, but this matches the C codes behavior
            let dec_part = dec_part.unwrap_or_default();

            let leading_traling_zeros =
                int_part.starts_with(&[b'0']) || dec_part.ends_with(&[b'0']);

            let is_numaric = |x: &[u8]| x.iter().all(|x| (b'0'..=b'9').contains(x));
            let numaric = is_numaric(int_part) && is_numaric(dec_part);
            let contains_no_digets = int_part.is_empty() && dec_part.is_empty();

            if !numaric
                || trailing_dot
                || leading_traling_zeros
                || parts.next().is_some()
                || contains_no_digets
                || int_part.len() > NON_NEGATIVE as usize - 1
            {
                if contents.contains(&b'\0') {
                    Err(KeyError::ContainsNull)
                } else {
                    Ok(Self::String(contents))
                }
            } else if negative {
                Ok(Self::Negative {
                    int_part: Box::new(int_part.iter().map(|x| b'9' - x + b'0')),
                    dec_part: Box::new(dec_part.iter().map(|x| b'9' - x + b'0')),
                })
            } else {
                Ok(Self::Positive { int_part, dec_part })
            }
        }
    }

    fn from_slice(slice: &'a [u8]) -> Self {
        let flag = slice[0];
        let data = &slice[1..];
        match flag {
            0 if data[0] == 0 => Self::Null,
            x if x & STRING_FLAG != 0 => {
                //TODO check if I should be including the null string.
                Self::String(data.split(|x| *x == 0).next().unwrap())
            }
            63 if data[0] == 255 => Self::Bug,
            x => {
                let non_negative = x & NON_NEGATIVE != 0;
                let int_len = if non_negative {
                    NON_NEGATIVE - 1 & x
                } else {
                    NON_NEGATIVE - 1 - x
                };

                let end_marker = if non_negative { 0 } else { 255 };

                let int_part = &data[0..int_len as usize];
                let dec_part = data[int_len as usize..]
                    .split(|x| *x == end_marker)
                    .next()
                    .unwrap();

                if int_part.is_empty() && dec_part.is_empty() {
                    Self::Zero
                } else if non_negative {
                    Self::Positive { int_part, dec_part }
                } else {
                    Self::Negative {
                        int_part: Box::new(int_part.iter().map(|x| *x)),
                        dec_part: Box::new(dec_part.iter().map(|x| *x)),
                    }
                }
            }
        }
    }
}

pub fn key_build(src: &CArrayString) -> Result<Key, KeyError> {
    let mut list = KeyList::new();
    list.push(src)?;
    Ok(Key::from(list.raw_keys()))
}

//TODO remove use of the string type.
//There is no guarantee that the M string will be valid urf8
pub fn key_extract(buff: &[u8], quote_strings: bool) -> Key {
    let tmp = KeyList::from_raw_key(buff);
    tmp.key_extract(quote_strings)
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod a_b_testing {
    use std::ptr::{from_mut, from_ref};

    use ffi::{UTIL_Key_Build, UTIL_Key_Extract, ERRMLAST, ERRZ1, ERRZ5, MAX_STR_LEN};

    use super::*;
    pub fn build(string: CArrayString) {
        let key = key_build(&string);
        let mut buffer = [0; MAX_STR_LEN as usize + 1];
        let len =
            unsafe { UTIL_Key_Build(from_ref(string.as_ref()).cast_mut(), buffer.as_mut_ptr()) };
        match key {
            Ok(key) => {
                assert_eq!(key[..], buffer[..len as usize]);
                //The c code should never write past key's len, but this is good to check.
                assert!(buffer[len as usize..].iter().all(|x| *x == 0));
            }
            Err(KeyError::InputToLarge) => {
                assert_eq!(-len, (ERRZ1 + ERRMLAST) as i16)
            }
            Err(KeyError::ContainsNull) => {
                assert_eq!(-len, (ERRZ5 + ERRMLAST) as i16)
            }
        }
    }

    pub fn extract(string: CArrayString) -> Result<(), KeyError> {
        //let result = unsafe { UTIL_Key_Build(from_ref(string.as_ref()).cast_mut(), input_buffer.as_mut_ptr()) };
        let key = super::key_build(&string)?;
        let mut output_buffer = [0; MAX_STR_LEN as usize + 1];
        let mut cnt = 0;

        //less then zero means there was a error building the key.
        let len = unsafe {
            UTIL_Key_Extract(
                key.clone().as_mut_ptr(),
                output_buffer.as_mut_ptr(),
                from_mut(&mut cnt),
            )
        };

        let output = key_extract(&key, false);
        assert_eq!(output[..], output_buffer[..len as usize]);
        Ok(())
    }

    pub fn string_key(keys: &[CArrayString]) -> Result<(), KeyError> {
        let mut key_list = KeyList::new();
        let _ = key_list.extend(keys.iter().cloned())?;
        let output = key_list.string_key();

        let mut output_buffer = [0; MAX_STR_LEN as usize + 1];

        //less then zero means there was a error building the key.
        let len = unsafe {
            ffi::UTIL_String_Key(
                key_list.as_raw(),
                output_buffer.as_mut_ptr(),
                keys.len() as i32,
            )
        };

        assert_eq!(&output, &output_buffer[..dbg!(len) as usize]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use a_b_testing::string_key;
    use ffi::{CSTRING, MAX_STR_LEN};
    use rstest::rstest;

    use super::*;

    impl From<&str> for CArrayString {
        fn from(value: &str) -> Self {
            let mut buf = [0; MAX_STR_LEN as usize + 1];
            buf[..value.len()].copy_from_slice(value.as_bytes());
            CArrayString(CSTRING {
                len: value.len().try_into().unwrap(),
                buf,
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
        super::a_b_testing::build(input.into());
        super::a_b_testing::extract(input.into())
            .expect("all of the test strings should produce valid keys");
    }

    #[test]
    fn build_key_max_size_tests() {
        let max_key = key_build(&"a".repeat(MAX_SUB_LEN as usize).as_str().into());
        assert!(max_key.is_ok());

        let max_key_pluse_one = key_build(&"a".repeat(MAX_SUB_LEN as usize + 1).as_str().into());
        assert_eq!(max_key_pluse_one, Err(KeyError::InputToLarge));
    }

    #[test]
    fn error_if_string_contains_null() {
        let bad_string = key_build(&"a\0b".into());
        assert_eq!(bad_string, Err(KeyError::ContainsNull));
    }

    #[test]
    fn build_key_int_to_large() {
        let src = &"1".repeat(NON_NEGATIVE as usize);
        let key = key_build(&src.as_str().into()).unwrap();
        assert!(matches!(
            KeyInternal::from_slice(&key),
            KeyInternal::String(_)
        ));
    }

    #[rstest]
    #[case(&["only"])]
    #[case(&[""])]
    #[case(&["9"])]
    #[case(&["-9"])]
    #[case(&["-9.0"])]
    #[case(&["f","s"])]
    fn key_extract_string(#[case] raw_keys: &[&str]) {
        //let keys = ["Test".into(), "Keys".into()];
        let keys = raw_keys
            .into_iter()
            .map(|x| (*x).into())
            .collect::<Vec<_>>();
        let keys = matches!(string_key(&keys), Ok(_));
    }
}
