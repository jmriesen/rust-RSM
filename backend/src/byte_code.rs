use crate::{
    Compile,
    conditional_jumps::JumpCodes,
    runtime::{
        Decode, StackAssembally,
        program_counter::{AssemblyDecoder, ProgramCounter},
    },
};
use ir::Expression;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

pub struct ByteCode(pub(crate) Vec<u8>);
impl Debug for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BiteCode")
            .field(&ProgramCounter::new(self))
            .finish()
    }
}

/// The JumpTarget is the bytecode location where we should resume execution from if we take the jump.
/// We store the JumpTarget before the code we might be jumping over.
/// This means we need to reserve space for the JumpTarget before we know the value of JumpTarget.
/// JumpTargetSlot represents a pointer to that reserved but not yet filled out jump target.
#[derive(Debug)]
pub struct JumpTargetSlot(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Location(pub usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct JumpOffset(pub i16);
impl Decode for JumpOffset {
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
    #[deprecated]
    pub fn push(&mut self, bite_code: u8) {
        self.0.push(bite_code);
    }

    pub fn write_jump(
        &mut self,
        JumpTargetSlot(location): JumpTargetSlot,
        Location(jump_to): Location,
    ) {
        let offset = (jump_to as i16 - location as i16).to_le_bytes();
        self.0[location - 2..location].copy_from_slice(&offset);
    }

    pub fn current_location(&self) -> Location {
        Location(self.0.len())
    }
    pub fn get_raw(self) -> Vec<u8> {
        self.0
    }
    #[deprecated]
    pub fn extend_old(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.0.extend(iter);
    }
    pub fn push_asm(&mut self, asm: StackAssembally) {
        asm.encode(self);
    }

    /// In general you should use the other jump methods
    /// This remains publicly exposed so that the For command can use it.
    pub fn reserve_jump(&mut self) -> JumpTargetSlot {
        self.0.push(0);
        self.0.push(0);
        JumpTargetSlot(self.0.len())
    }

    pub fn conditional_jump<T>(
        &mut self,
        condition: &Expression,
        conditional_code: impl Fn(&mut Self) -> T,
    ) -> T {
        // Put condition expression on the stack
        condition.compile(self, &crate::expression::ExpressionContext::Eval);
        // add jump command
        self.push(JumpCodes::Conditional as u8);
        let conditional_jump = self.reserve_jump();
        // Compile code to be conditionally included.
        let conditional_code_return = conditional_code(self);
        // store target location.
        self.write_jump(conditional_jump, self.current_location());
        conditional_code_return
    }
    pub fn unconditional_jump(&mut self) -> JumpTargetSlot {
        self.push(JumpCodes::Unconditional as u8);
        self.reserve_jump()
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
