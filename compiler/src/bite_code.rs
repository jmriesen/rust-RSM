pub struct BiteCode(Vec<u8>);

impl BiteCode {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, bite_code: u8) {
        self.0.push(bite_code);
    }
    pub fn reserve_jump(&mut self) -> usize {
        self.0.push(0);
        self.0.push(0);
        self.0.len()
    }

    pub fn write_jump(&mut self, location: usize, jump_to: usize) {
        let offset = (jump_to as i16 - location as i16).to_le_bytes();
        self.0[location - 2..location].copy_from_slice(&offset);
    }

    pub fn current_location(&self) -> usize {
        self.0.len()
    }
    pub fn get_raw(self) -> Vec<u8> {
        self.0
    }
    pub fn extend(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.0.extend(iter);
    }
    pub fn pop(&mut self) {
        self.0.pop();
    }
}
