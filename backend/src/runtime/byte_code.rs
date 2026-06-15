use std::{fmt::Debug, ops::Range};

use crate::runtime::{Decode, StackAssembally};

#[derive(Clone, Copy, Debug)]
pub struct Location(pub usize);
#[derive(Clone, Copy, Debug)]
pub struct ReletiveJump(pub i16);
impl Decode for ReletiveJump {
    fn decode(bytes: &[u8]) -> Option<(Self, &[u8])> {
        if let ([first, second], tail) = bytes.split_at(2) {
            Some((Self(i16::from_le_bytes([*first, *second])), tail))
        } else {
            None
        }
    }
}

impl ReletiveJump {
    pub fn adjust(&mut self, offset: i16) {
        self.0 += offset
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
impl<'a> ByteCode<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            program_counter: Location(0),
        }
    }
    fn try_decode<T: Decode>(&mut self) -> Option<T> {
        if let Some((value, tail)) = T::decode(&self.source[self.program_counter.0..]) {
            self.program_counter.0 = self.source.len() - tail.len();
            Some(value)
        } else {
            None
        }
    }
    pub fn next(&mut self) -> StackAssembally {
        //Starting with none to get nice vertical alignment
        //Trusting that the compiler will optimize it away.
        None.or_else(|| self.try_decode().map(StackAssembally::WriteCode))
            .or_else(|| self.try_decode().map(StackAssembally::Literal))
            .or_else(|| self.try_decode().map(StackAssembally::UnaryOp))
            .or_else(|| self.try_decode().map(StackAssembally::BinaryOpCode))
            .or_else(|| self.try_decode().map(StackAssembally::EndLine))
            .or_else(|| self.try_decode().map(StackAssembally::EndCommand))
            .or_else(|| {
                self.try_decode().map(|set| StackAssembally::ForSet {
                    set: set,
                    start_address: self.program_counter,
                })
            })
            .or_else(|| self.try_decode().map(StackAssembally::ForStart))
            .or_else(|| self.try_decode().map(StackAssembally::ForEnd))
            .or_else(|| self.try_decode().map(StackAssembally::NoOpCode))
            .or_else(|| self.try_decode().map(StackAssembally::TEMP))
            .expect("Provided source was invalid/corruped")
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
