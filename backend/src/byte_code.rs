use crate::runtime::{
    LegacyEncoding, StackAssembally,
    program_counter::{AssemblyDecoder, ProgramCounter},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

pub struct ByteCode(Vec<u8>);
impl Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BiteCode")
            .field(&ProgramCounter::new(self))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Location(pub usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct JumpOffset(pub i16);
impl LegacyEncoding for JumpOffset {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        Some(Self(i16::from_le_bytes(decoder.consume_n())))
    }
    fn encode(&self, byte_code: &mut self::ByteCode) {
        use crate::byte_code::EncoderExt;
        byte_code.extend(i16::to_le_bytes(self.0));
    }
}

impl Add<JumpOffset> for Location {
    type Output = Location;
    fn add(self, rhs: JumpOffset) -> Self::Output {
        Location(
            usize::try_from(
                isize::try_from(self.0).expect("jump pointer overflow issue.") + isize::from(rhs.0),
            )
            .expect("jump pointer overflow issue."),
        )
    }
}

impl AddAssign<JumpOffset> for Location {
    fn add_assign(&mut self, rhs: JumpOffset) {
        *self = *self + rhs;
    }
}

impl Default for ByteCode {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteCode {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn current_location(&self) -> Location {
        Location(self.0.len())
    }

    pub fn push_asm(&mut self, asm: StackAssembally) {
        asm.encode(self);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn append(&mut self, other: ByteCode) {
        self.0.extend(other.0);
    }

    /// Try Not to use this.
    /// Will Likely go away in the future.
    pub fn get_raw(&self) -> &[u8] {
        &self.0
    }
}
pub trait EncoderExt {
    fn push_raw(&mut self, bite_code: u8);
    fn extend(&mut self, iter: impl IntoIterator<Item = u8>);
}

impl EncoderExt for ByteCode {
    fn push_raw(&mut self, bite_code: u8) {
        self.0.push(bite_code);
    }
    fn extend(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.0.extend(iter);
    }
}
