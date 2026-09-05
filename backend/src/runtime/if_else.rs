use symbol_table::SymbolTable;

use crate::runtime::{
    Decode, Job, OpCode, RuntimeError,
    r#for::ForFrame,
    program_counter::{self, AssemballyDecoder, ProgramCounter},
};

impl<'a> Job<'a> {
    /// Jump handling for if/else statements.
    /// NOTE: This computes the jump, not if we should jump.
    pub(crate) fn if_jump(
        for_stack: &mut Vec<ForFrame>,
        symbol_table: &mut SymbolTable,
        pc: &mut ProgramCounter<'_>,
        error: &mut Option<RuntimeError>,
    ) {
        if !for_stack.is_empty() {
            Self::loop_condition_check_slash_increment(for_stack, symbol_table, pc, error);
        } else {
            pc.advance_to_next_line();
        }
    }
}
OpCode! {JumpIfFalseCode=5}
#[derive(Debug)]
pub struct JumpIfFalse {
    pub target: program_counter::Location,
}
impl Decode for JumpIfFalse {
    fn decode(decoder: &mut AssemballyDecoder<'_>) -> Option<Self> {
        JumpIfFalseCode::decode(decoder)?;
        Some(Self {
            target: Decode::decode(decoder)?,
        })
    }
}
