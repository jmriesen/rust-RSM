pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;
pub use write::Write;

use crate::bite_code::BiteCode;

pub enum Command {}
impl Command {
    pub fn new(sitter: &lang_model::command, source_code: &str) -> Option<Self> {
        None
    }
    pub fn compile(&self, _: &mut BiteCode) {}
}
