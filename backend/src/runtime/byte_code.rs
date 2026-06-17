use std::{fmt::Debug, ops::Range};

use crate::runtime::{Decode, StackAssembally, StackAssemballyTrait};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location(pub usize);
#[derive(Clone, Copy, Debug)]
pub struct Jump(pub Location);
impl Decode for Jump {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        let jump_distance = i16::from_le_bytes(decoder.consume_n());
        let Location(here) = decoder.program_counter;
        Some(Self(Location(
            here + usize::try_from(jump_distance).expect(
                "Currently only supporting forward jumps. May change if new funcionality needs it.",
            ),
        )))
    }
}

#[derive(Clone)]
pub struct ByteCode<'a> {
    source: &'a [u8],
    program_counter: Location,
}
impl<'a> Debug for ByteCode<'a> {
    #[cfg_attr(test, mutants::skip)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ByteCode")
            .field("program_counter", &self.program_counter.0)
            .field("parsed", &self.dbg_helper())
            .finish()
    }
}

/// Responsible for parsing assembly instruction.
/// This **is allowed** to decode partial instructions.
pub struct AssemballyDecoder<'a> {
    source: &'a [u8],
    program_counter: Location,
}
impl<'a> AssemballyDecoder<'a> {
    pub fn tail(&self) -> &'a [u8] {
        &self.source[self.program_counter.0..]
    }
    pub fn consume(&mut self, bytes: usize) -> &'a [u8] {
        let tail = self.tail();
        assert!(tail.len() >= bytes);
        self.program_counter.0 += bytes;
        &tail[..bytes]
    }
    pub fn consume_n<const BYTES: usize>(&mut self) -> [u8; BYTES] {
        let tail = &self.source[self.program_counter.0..];
        assert!(tail.len() >= BYTES);
        self.program_counter.0 += BYTES;
        tail[..BYTES]
            .try_into()
            .expect("bounds have already ben checked")
    }
}

impl<'a> ByteCode<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            program_counter: Location(0),
        }
    }

    /// Responsible for parsing **full** assembly instruction.
    /// This is atomic it will decode a full instruction and update the program counter,
    /// Or it will fail without modifying self's internal state.
    pub(crate) fn try_decode<T: StackAssemballyTrait>(&mut self) -> Option<T> {
        let mut decoder = AssemballyDecoder {
            source: self.source,
            program_counter: self.program_counter,
        };
        if let Some(value) = T::decode(&mut decoder) {
            self.program_counter = decoder.program_counter;
            Some(value)
        } else {
            None
        }
    }

    pub fn end(&self) -> bool {
        self.program_counter.0 == self.source.len()
    }

    #[cfg_attr(test, mutants::skip)]
    fn dbg_helper(&self) -> Vec<(bool, Range<usize>, StackAssembally, &'a [u8])> {
        let mut scrach = self.clone();
        scrach.program_counter.0 = 0;
        let mut vec: Vec<(bool, Range<usize>, StackAssembally, &'a [u8])> = vec![];
        while !scrach.end() {
            let start = scrach.program_counter.0;
            let asm = scrach.next();
            let end = scrach.program_counter.0;
            vec.push((
                (start..end).contains(&self.program_counter.0),
                (start..end),
                asm,
                &scrach.source[start..end],
            ));
        }
        vec
    }

    pub fn jump_absolute(&mut self, location: Location) {
        self.program_counter = location
    }
}
