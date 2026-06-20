use ir::{
    Expression,
    commands::{Command, PostCondition, Quit},
};
use lang_model::QuitCommand;

use crate::TreeSitterParser;
pub(crate) fn new(sitter: &QuitCommand<'_>, source_code: &str) -> Command {
    assert!(sitter.args().len() <= 1, "quit can only have one arg");
    Command::Quit(PostCondition {
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
    })
}

