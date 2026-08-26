use symbol_table::SymbolTable;

use crate::runtime::{Job, RuntimeError, r#for::ForFrame, program_counter::ProgramCounter};

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
            Self::loop_body_post_check(for_stack, symbol_table, pc, error);
        } else {
            pc.advance_to_next_line();
        }
    }
}
