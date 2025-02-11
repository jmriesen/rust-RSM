use ir::Expression;

use crate::Compile;

pub struct BiteCode(Vec<u8>);

pub struct JumpLocation(usize);
pub struct Location(usize);

impl BiteCode {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, bite_code: u8) {
        self.0.push(bite_code);
    }

    pub fn write_jump(
        &mut self,
        JumpLocation(location): JumpLocation,
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
    pub fn extend(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.0.extend(iter);
    }

    /// In general you should use the other jump methods
    /// This remains publicly exposed so that the For command can use it.
    pub fn reserve_jump(&mut self) -> JumpLocation {
        self.0.push(0);
        self.0.push(0);
        JumpLocation(self.0.len())
    }

    pub fn conditional_jump<T>(
        &mut self,
        condition: &Expression,
        conditional_code: impl Fn(&mut Self) -> T,
    ) -> T {
        condition.compile(self, &crate::expression::ExpressionContext::Eval);
        self.push(ffi::JMP0);
        let post_condition_jump = self.reserve_jump();
        let value = conditional_code(self);
        self.write_jump(post_condition_jump, self.current_location());
        value
    }
    pub fn unconditional_jump(&mut self) -> JumpLocation {
        self.push(ffi::JMP);
        self.reserve_jump()
    }
}
