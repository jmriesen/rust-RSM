use ir::{
    Expression,
    commands::{Command, PostCondition, close::Close},
};
use lang_model::CloseCommand;

use crate::{ParsingError, TreeSitterParser};

pub fn new(sitter: &CloseCommand, source_code: &str) -> Result<Command, ParsingError> {
    if sitter.args().is_empty() {
        Err(ParsingError::CloseRequiresArgs(sitter.node().range()))
    } else {
        Ok(Command::Close(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),
            value: sitter
                .args()
                .iter()
                .map(|x| Expression::new(x, source_code))
                .map(Close)
                .collect(),
        }))
    }
}
