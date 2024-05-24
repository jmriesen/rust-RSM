#![allow(dead_code)]
use ffi::CSTRING;

//TODO I will eventually want to be able to write a string in place.
type Key = Vec<u8>;

static STRING_FLAG: u8 = 0b1000_0000;
static NON_NEGATIVE: u8 = 0b100_0000;

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
    String(&'a [u8]),
}

impl<'a> KeyInternal<'a> {
    fn new(src: &'a CSTRING) -> Self {
        if src.len == 0 {
            Self::Null
        } else if src.len == 1 && src.buf[0] == b'0' {
            Self::Zero
        } else {
            //attempt to parse as a number
            let contents = &src.buf[..src.len as usize];
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
            let dec_part = parts.next().unwrap_or_default();

            let leading_traling_zeros =
                int_part.starts_with(&[b'0']) || dec_part.ends_with(&[b'0']);

            let is_numaric = |x: &[u8]| x.iter().all(|x| (b'0'..=b'9').contains(x));
            let numaric = is_numaric(int_part) && is_numaric(dec_part);

            if !numaric || leading_traling_zeros || parts.next().is_some() {
                Self::String(contents)
            } else if negative {
                Self::Negative {
                    int_part: Box::new(int_part.iter().map(|x| b'9' - x + b'0')),
                    dec_part: Box::new(dec_part.iter().map(|x| b'9' - x + b'0')),
                }
            } else {
                Self::Positive { int_part, dec_part }
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

fn key_build(src: &CSTRING) -> Key {
    let mut key = Key::new();
    let internal_key = KeyInternal::new(&src);
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
    };
    //Adding end markers
    //NOTE Negatives use 255 as an end mark for some reason.
    //at some point when I understand 9's complement better I should se if I can remove this inconsistency
    key.push(end_mark.unwrap_or(b'\0'));
    key
}

//TODO remove use of the string type.
//There is no guarantee that the M string will be valid urf8
fn key_extract(buff: &[u8]) -> Key {
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
    #[case("test string")]
    #[case("")]
    #[case("0")]
    #[case("10")]
    #[case("10E")]
    #[case("10.4")]
    #[case(".4")]
    #[case("10.4E")]
    #[case("10.0")]
    #[case("010")]
    #[case("10.5.")]
    #[case("-10")]
    #[case("-10E")]
    #[case("-10.4")]
    #[case("-.4")]
    #[case("-10.4E")]
    #[case("-10.0")]
    #[case("-010")]
    #[case("-10.5.")]
    fn simple_string(#[case] input: &str) {
        let string = create_cstring(dbg!(input));
        let key = key_build(&string);
        let mut buffer = [0; MAX_STR_LEN as usize + 1];
        let len = unsafe { UTIL_Key_Build(from_ref(&string).cast_mut(), buffer.as_mut_ptr()) };
        assert_eq!(key[..], buffer[..len as usize]);
        //The c code should never write past key's len, but this is good to check.
        assert!(buffer[len as usize..].iter().all(|x| *x == 0));
    }

    #[rstest]
    #[case("")]
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
    #[case("-10")]
    #[case("-10E")]
    #[case("-10.4")]
    #[case("-.4")]
    #[case("-10.4E")]
    #[case("-10.0")]
    #[case("-010")]
    #[case("-10.5.")]
    fn test_extraction(#[case] input: &str) {
        use ffi::UTIL_Key_Extract;

        let string = create_cstring(dbg!(input));
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
