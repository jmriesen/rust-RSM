use crate::key::KeyRef;
use ffi::MAX_SUB_LEN;

/// Keys are stored in a special format to facilitate sorting.
/// They start with a discriminant.
/// [0,0] -> null
/// [64,0] -> zero
/// [INT_ZERO_POINT+x, ..x bytes.., ..., 0] -> positive number; x is # of integer digits (base 10)
/// the digits of a negative number are stored as nines complement.
/// [INT_ZERO_POINT-x, ..x bytes.., ..., 0] -> negative number; x is # of integer digits (base 10)
/// [STRING_FLAG,..., 0] -> string (if numbers are to large they are stored as strings.

static STRING_FLAG: u8 = 0b1000_0000;
static INT_ZERO_POINT: u8 = 0b100_0000;

/// Only 7 bytes are allocated to store the size of the int portion of a number.
/// any number that takes more integer digits will be stored as a string.
pub static MAX_INT_SEGMENT_SIZE: usize = INT_ZERO_POINT as usize - 1;

use super::{CArrayString, KeyError, KeyIter, KeyList};

/// represents a key parsed out into its individual parts.
pub enum ParsedKey<'a> {
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

impl<'a> ParsedKey<'a> {
    pub fn new(src: &'a CArrayString) -> Result<Self, KeyError> {
        let contents = src.content();
        if contents.len() > MAX_SUB_LEN as usize {
            //TODO consider reevaluating where this should be enforced.
            //If the goal of this is to prevent a buffer overflow then it should really be owned by
            //KeyList
            Err(KeyError::InputToLarge)
        } else if contents == &[] {
            Ok(Self::Null)
        } else if contents == &[b'0'] {
            Ok(Self::Zero)
        } else if contents == &[b'-', b'.'] {
            Ok(Self::Bug)
        } else if contents.contains(&b'\0') {
            Err(KeyError::ContainsNull)
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

            let multiple_decimal_points = parts.next().is_some();
            //TODO remove the !negative, this was caused by a bug in the C code.
            //The issue has been reported upstream.
            let trailing_dot = dec_part == Some(&[]) && !negative;
            let dec_part = dec_part.unwrap_or_default();

            let leading_trailing_zeros =
                int_part.starts_with(&[b'0']) || dec_part.ends_with(&[b'0']);

            let is_numeric = |x: &[u8]| x.iter().all(|x| (b'0'..=b'9').contains(x));
            let numeric = is_numeric(int_part) && is_numeric(dec_part);
            let contains_no_digits = int_part.is_empty() && dec_part.is_empty();

            if !numeric
                || trailing_dot
                || leading_trailing_zeros
                || multiple_decimal_points
                || contains_no_digits
                || int_part.len() > MAX_INT_SEGMENT_SIZE
            {
                Ok(Self::String(contents))
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

    // I don't really like returning the tail as part of the tuple
    // but it will work for now.
    pub fn from_key_ref(key: KeyRef<'a>) -> Self {
        let flag = key.0[0];
        let data = &key.0[1..];
        match flag {
            0 if data.is_empty() => Self::Null,
            x if x & STRING_FLAG != 0 => Self::String(&data),
            63 if data.is_empty() => Self::Bug,
            x => {
                let non_negative = x & INT_ZERO_POINT != 0;
                let int_len = if non_negative {
                    INT_ZERO_POINT - 1 & x
                } else {
                    INT_ZERO_POINT - 1 - x
                };

                let int_part = &data[0..int_len as usize];
                let dec_part = &data[int_len as usize..];

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

    //TODO consider removing allocation.
    //There is nothing inherent to this method that requires allocation.
    pub fn to_external(self, quote_strings: bool) -> Vec<u8> {
        match self {
            ParsedKey::Null => {
                if quote_strings {
                    vec![b'"', b'"']
                } else {
                    vec![]
                }
            }
            ParsedKey::Zero => vec![b'0'],
            ParsedKey::Positive { int_part, dec_part } => {
                let mut key = Vec::from(int_part);
                if !dec_part.is_empty() {
                    key.push(b'.');
                    key.extend(dec_part);
                }
                key
            }
            ParsedKey::Negative { int_part, dec_part } => {
                let mut key = vec![b'-'];
                key.extend(int_part.map(|x| b'9' + b'0' - x));
                let mut dec_part = dec_part.peekable();
                if dec_part.peek().is_some() {
                    key.push(b'.');
                    key.extend(dec_part.map(|x| b'9' + b'0' - x));
                }
                key
            }
            ParsedKey::String(string) => {
                if quote_strings {
                    let mut output = vec![b'"'];
                    let ends_with_quote =
                        *string.last().expect("empty strings are handle as null") == b'"';
                    //strings are escaped by adding a second " so "\"" = """"
                    let segments = string.split_inclusive(|x| *x == b'"');
                    for segment in segments {
                        output.extend(segment);
                        output.push(b'"');
                    }

                    //if the last segment ends in a quote we need to
                    //add an extra one due to how split_inclusive works.
                    if ends_with_quote {
                        output.push(b'"');
                    }
                    output
                } else {
                    Vec::from(string)
                }
            }
            ParsedKey::Bug => {
                vec![b'-']
            }
        }
    }
}

impl KeyList {
    pub fn push(&mut self, src: &CArrayString) -> Result<(), KeyError> {
        let internal_key = ParsedKey::new(&src)?;
        let end_mark = match internal_key {
            ParsedKey::Null => {
                self.0.push(b'\0');
                None
            }
            ParsedKey::Zero => {
                self.0.push(INT_ZERO_POINT);
                None
            }
            ParsedKey::Positive { int_part, dec_part } => {
                self.0.push(INT_ZERO_POINT + int_part.len() as u8);
                self.0.extend(int_part);
                self.0.extend(dec_part);
                None
            }
            ParsedKey::Negative { int_part, dec_part } => {
                self.0.push(INT_ZERO_POINT - 1 - int_part.len() as u8);
                //TODO figure out how this complement is supposed to work.
                self.0.extend(int_part);
                self.0.extend(dec_part);
                Some(255)
            }
            ParsedKey::String(contents) => {
                self.0.push(STRING_FLAG);
                self.0.extend(contents);
                None
            }
            ParsedKey::Bug => {
                //I think this case is a bug in the C code
                //For now I am replicating the behavior
                //Encoding as -0
                self.0.extend([63, 255]);
                return Ok(());
            }
        };
        //Adding end markers
        //NOTE Negatives use 255 as an end mark for some reason.
        //at some point when I understand 9's complement better I should see if I can remove this
        self.0.push(end_mark.unwrap_or(b'\0'));
        //TODO test size is accurate
        self.0[0] = self.0.len() as u8;
        Ok(())
    }
}

impl<'a> std::iter::Iterator for KeyIter<'a> {
    type Item = KeyRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tail.first() {
            //special handling for null.
            //since the flagged value == the end mark value
            //we need to manually split the string.
            Some(b'\0') => {
                // null internally stored as [b'\0',b'\0']
                let (key, tail) = (&self.tail[..1], &self.tail[2..]);
                self.tail = tail;
                Some(KeyRef(key))
            }
            Some(x) => {
                let end_mark = match *x {
                    x if x & STRING_FLAG != 0 => b'\0',
                    x if x & INT_ZERO_POINT != 0 => b'\0',
                    //negative numbers
                    _ => 255,
                };
                let (key, tail) = self
                    .tail
                    .split_once(|x| *x == end_mark)
                    .expect("all keys must contain an endmark");
                self.tail = tail;
                Some(KeyRef(key))
            }
            None => None,
        }
    }
}

impl<'a> Ord for KeyRef<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let len = self.0.len().min(other.0.len());

        match self.0[..len].cmp(&other.0[..len]) {
            //NOTE If the prefixes are the same the longer one comes first.
            std::cmp::Ordering::Equal => self.0.len().cmp(&other.0.len()).reverse(),
            x => x,
        }
    }
}
impl<'a> PartialOrd for KeyRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}