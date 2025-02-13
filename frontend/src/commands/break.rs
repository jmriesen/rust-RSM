use ir::{
    Expression,
    commands::{Command, PostCondition, r#break::Break},
};
use lang_model::BrakeCommand;

use crate::TreeSitterParser;

pub fn new(sitter: &BrakeCommand<'_>, source_code: &str) -> Command {
    Command::Break(PostCondition {
        condition: sitter
            .post_condition()
            .map(|x| Expression::new(&x, source_code)),
        value: if sitter.args().is_empty() {
            Break::ArgumentLess
        } else {
            Break::Arg(
                sitter
                    .args()
                    .iter()
                    .map(|x| Expression::new(&x, source_code))
                    .collect(),
            )
        },
    })
}
