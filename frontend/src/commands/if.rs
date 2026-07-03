use ir::{
    Expression,
    commands::{Command, r#if::If},
};
use lang_model::IfCommand;

use crate::{ParsingError, TreeSitterParser};

pub fn new(sitter: &IfCommand, source_code: &str) -> Result<Command, ParsingError> {
    if sitter.args().is_empty() {
        Err(ParsingError::IfReqiresArgs(sitter.node().range()))
    } else {
        Ok(Command::If(
            sitter
                .args()
                .iter()
                .map(|x| Expression::new(x, source_code))
                .map(If)
                .collect(),
        ))
    }
}
