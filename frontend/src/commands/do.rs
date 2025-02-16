use ir::{
    Expression, ExtrinsicFunction,
    commands::{Command, PostCondition, r#do::Do},
};
use lang_model::DoCommand;

use crate::TreeSitterParser;

pub fn new(sitter: &DoCommand, source_code: &str) -> Command {
    Command::Do(PostCondition {
        condition: sitter
            .post_condition()
            .map(|x| Expression::new(&x, source_code)),
        value: if sitter.args().is_empty() {
            Do::ArgumentLess
        } else {
            Do::FunctionCall(
                sitter
                    .args()
                    .iter()
                    .map(|x| PostCondition {
                        condition: x.post_condition().map(|x| Expression::new(&x, source_code)),
                        value: ExtrinsicFunction::new(&x.function(), source_code),
                    })
                    .collect(),
            )
        },
    })
}
