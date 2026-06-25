use ir::{
    Expression,
    commands::{Command, PostCondition, Quit},
};
use lang_model::QuitCommand;

use crate::{ParsingError, TreeSitterParser};
pub(crate) fn new(sitter: &QuitCommand<'_>, source_code: &str) -> Result<Command, ParsingError> {
    if sitter.args().len() > 1 {
        Err(ParsingError::QuitExtraArgs(sitter.node().range()))
    } else {
        Ok(Command::Quit(PostCondition {
            condition: sitter
                .post_condition()
                .map(|x| Expression::new(&x, source_code)),

            value: Quit(
                sitter
                    .args()
                    .iter()
                    .map(|x| Expression::new(x, source_code))
                    .next(),
            ),
        }))
    }
}
