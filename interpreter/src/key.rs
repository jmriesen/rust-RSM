#![allow(dead_code)]
use ffi::{CSTRING, MAX_SUB_LEN};

//TODO I will eventually want to be able to write a string in place.
type Key = Vec<u8>;

static STRING_FLAG: u8 = 0b1000_0000;
static NON_NEGATIVE: u8 = 0b100_0000;

#[derive(Debug,PartialEq)]
pub enum KeyError {
    InputToLarge,
    ContainsNull
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
    fn new(src: &'a CSTRING) -> Result<Self, KeyError> {
        let contents = &src.buf[..src.len as usize];
        if src.len > MAX_SUB_LEN as u16 {
            //consider reevaluating this.
            //the restrictions seems a bit arbitrary
            Err(KeyError::InputToLarge)
        }else if contents == &[] {
            Ok(Self::Null)
        } else if  contents == &[b'0'] {
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
                || int_part.len() > NON_NEGATIVE as usize -1
            {
                if contents.contains(&b'\0'){
                    Err(KeyError::ContainsNull)
                }else{
                    Ok(Self::String(contents))
                }
            }else if negative {
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
            0 => Self::Null,
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

pub fn key_build(src: &CSTRING) -> Result<Key, KeyError> {
    let mut key = Key::new();
    let internal_key = KeyInternal::new(&src)?;
    let end_mark = match internal_key {
        KeyInternal::Null => {
            key.push(b'\0');
            None
        }
        KeyInternal::Zero => {
            key.push(NON_NEGATIVE);
            None
        }
        KeyInternal::Positive { int_part, dec_part } => {
            key.push(NON_NEGATIVE + int_part.len() as u8);
            key.extend(int_part);
            key.extend(dec_part);
            None
        }
        KeyInternal::Negative { int_part, dec_part } => {
            key.push(63 - int_part.len() as u8);
            //TODO figure out how this complement is supposed to work.
            key.extend(int_part);
            key.extend(dec_part);
            Some(255)
        }
        KeyInternal::String(contents) => {
            key.push(STRING_FLAG);
            key.extend(contents);
            None
        }
        KeyInternal::Bug => {
            //I think this case is a bug in the C code
            //For now I am replicating the behavior
            //Encoding as -0
            key.extend([63, 255]);
            return Ok(key);
        }
    };
    //Adding end markers
    //NOTE Negatives use 255 as an end mark for some reason.
    //at some point when I understand 9's complement better I should se if I can remove this inconsistency
    key.push(end_mark.unwrap_or(b'\0'));
    Ok(key)
}

//TODO remove use of the string type.
//There is no guarantee that the M string will be valid urf8
pub fn key_extract(buff: &[u8]) -> Key {
    match KeyInternal::from_slice(buff) {
        KeyInternal::Null => {
            vec![]
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
        KeyInternal::String(string) => Vec::from(string),
        KeyInternal::Bug => {
            vec![b'-']
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::{from_mut, from_ref};

    use ffi::{UTIL_Key_Build, CSTRING, MAX_STR_LEN};
    use rstest::rstest;

    use super::*;

    fn create_cstring(ascii: &str) -> CSTRING {
        assert!(ascii.is_ascii());
        let mut buf = [0; MAX_STR_LEN as usize + 1];
        buf[..ascii.len()].copy_from_slice(ascii.as_bytes());
        CSTRING {
            len: ascii.len().try_into().unwrap(),
            buf,
        }
    }

    //This has limited side effect and may a good candidate for fuz testing.
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
    fn build_key_a_b(#[case] input: &str) {
        let string = create_cstring(input);
        let key = key_build(&string).unwrap();
        let mut buffer = [0; MAX_STR_LEN as usize + 1];
        let len = unsafe { UTIL_Key_Build(from_ref(&string).cast_mut(), buffer.as_mut_ptr()) };
        assert_eq!(key[..], buffer[..dbg!(len) as usize]);
        //The c code should never write past key's len, but this is good to check.
        assert!(buffer[len as usize..].iter().all(|x| *x == 0));
    }

    fn build_key_max_size_tests() {
        let max_key = key_build(&create_cstring(&"a".repeat(MAX_SUB_LEN as usize)));
        assert!(max_key.is_ok());

        let max_key_pluse_one = key_build(&create_cstring(&"a".repeat(MAX_SUB_LEN as usize + 1)));
        assert_eq!(max_key_pluse_one,Err(KeyError::InputToLarge));
    }

    fn error_if_string_contains_null() {
        let bad_string = key_build(&create_cstring(&"a\0b"));
        assert_eq!(bad_string,Err(KeyError::ContainsNull));
    }

    fn build_key_int_to_large(){
        let src = &"1".repeat(NON_NEGATIVE as usize);
        let key = key_build(&create_cstring(&src)).unwrap();
        if let KeyInternal::String(_)=  KeyInternal::from_slice(&key){
            //large numbers are encoded as strings
        }else{
            panic!();
        };

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
    #[case("-10")]
    #[case("-10E")]
    #[case("-10.4")]
    #[case("-.4")]
    #[case("-.")]
    #[case("-4.")]
    #[case("-10.4E")]
    #[case("-10.0")]
    #[case("-010")]
    #[case("-10.5.")]
    fn test_extraction(#[case] input: &str) {
        use ffi::UTIL_Key_Extract;

        let string = create_cstring(input);
        let mut input_buffer = [0; MAX_STR_LEN as usize + 1];
        let _ = unsafe { UTIL_Key_Build(from_ref(&string).cast_mut(), input_buffer.as_mut_ptr()) };
        let mut output_buffer = [0; MAX_STR_LEN as usize + 1];
        let mut cnt = 0;
        //TODO figure out what temp correspond to.
        let len = unsafe {
            UTIL_Key_Extract(
                input_buffer.as_mut_ptr(),
                output_buffer.as_mut_ptr(),
                from_mut(&mut cnt),
            )
        };

        let output = key_extract(&input_buffer);
        assert_eq!(output[..], output_buffer[..len as usize]);
    }
}
