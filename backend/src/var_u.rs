use crate::{BiteCode, Compile};
const VAR_LEN: i32 = 32;

impl Compile for &str {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        bite_code.extend(
            self.as_bytes()
                .iter()
                .cloned()
                .chain(std::iter::repeat(0))
                .take(VAR_LEN as usize),
        )
    }
}
