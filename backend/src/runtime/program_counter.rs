use std::fmt::Debug;

use crate::runtime::{Decode, StackAssembally, StackAssemblyTrait};

/// A location in the byte code.
/// Used by jumps and program counters
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location(pub usize);
impl Decode for Location {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let jump_distance = i16::from_le_bytes(decoder.consume_n());
        let Location(here) = decoder.program_counter;
        Some(Self(
            here + usize::try_from(jump_distance).expect(
                "Currently only supporting forward jumps. May change if new functionality needs it.",
            ),
        ))
    }
}

/// Responsible for parsing assembly instruction.
/// This **is allowed** to decode partial instructions.
pub struct AssemblyDecoder<'a> {
    source: &'a [u8],
    program_counter: Location,
}
impl<'a> AssemblyDecoder<'a> {
    pub fn consume(&mut self, bytes: usize) -> &'a [u8] {
        let start = self.program_counter.0;
        self.program_counter.0 += bytes;
        let end = self.program_counter.0;
        let content = &self.source[start..end];
        assert!(
            content.len() == bytes,
            "There should be enough bytes remaining if the code was compile/decode properly"
        );
        content
    }
    pub fn consume_n<const BYTES: usize>(&mut self) -> [u8; BYTES] {
        self.consume(BYTES)
            .try_into()
            .expect("len is already checked in consume")
    }
    pub fn current_location(&self) -> Location {
        self.program_counter
    }
}

#[derive(Clone)]
pub struct ProgramCounter<'a> {
    source: &'a [u8],
    program_counter: Location,
}

impl<'a> ProgramCounter<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            program_counter: Location(0),
        }
    }

    /// Responsible for parsing **full** assembly instruction.
    /// This is atomic it will decode a full instruction and update the program counter,
    /// Or it will fail without modifying self's internal state.
    pub(crate) fn try_decode<T: StackAssemblyTrait>(&mut self) -> Option<T> {
        let mut decoder = AssemblyDecoder {
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

    pub fn has_next(&self) -> bool {
        self.program_counter.0 == self.source.len()
    }

    pub fn jump(&mut self, location: Location) {
        self.program_counter = location
    }
    pub fn advance_to_next_line(&mut self) {
        loop {
            if let StackAssembally::EndLine(_) = self
                .next()
                .expect("Advance to next line assumes we are in a line so their must only be an end of line. (may not apply during routine front/end material, but we this function logically should never be called then)")
            {
                break;
            };
        }
    }

    pub(crate) fn current_location(&self) -> Location {
        self.program_counter
    }
}

mod debug {
    use crate::runtime::{StackAssembally, program_counter::ProgramCounter};
    use std::ops::Range;

    #[allow(unused)]
    struct InstructionInfo<'a> {
        byte_code_range: Range<usize>,
        byte_code: &'a [u8],
        stack_asm: StackAssembally,
    }

    struct InstructionDebugIter<'a>(ProgramCounter<'a>);

    impl<'a> Iterator for InstructionDebugIter<'a> {
        type Item = InstructionInfo<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            let start = self.0.program_counter.0;
            let asm = self.0.next()?;
            let end = self.0.program_counter.0;

            let byte_code_range = start..end;

            Some(InstructionInfo {
                byte_code: &self.0.source[byte_code_range.clone()],
                byte_code_range,
                stack_asm: asm,
            })
        }
    }

    impl<'a> std::fmt::Debug for ProgramCounter<'a> {
        #[cfg_attr(test, mutants::skip)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ByteCode")
                .field("program_counter", &self.program_counter.0)
                .field(
                    "parsed",
                    &InstructionDebugIter(self.clone())
                        //Pulling out just what is useful for the current debugging
                        //session.
                        .map(|x| x.stack_asm)
                        .collect::<Vec<_>>(),
                )
                .finish()
        }
    }
}
