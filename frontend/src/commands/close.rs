use ir::{
    Expression,
    commands::{Command, PostCondition, close::Close},
};
use lang_model::CloseCommand;

use crate::TreeSitterParser;

pub fn new(sitter: &CloseCommand, source_code: &str) -> Command {
    assert!(
        sitter.args().len() > 0,
        "Close always takes at least one argument"
    );
    Command::Close(PostCondition {
        condition: sitter
            .post_condition()
            .map(|x| Expression::new(&x, source_code)),
        value: sitter
            .args()
            .iter()
            .map(|x| Expression::new(x, source_code))
            .map(|x| Close(x))
            .collect(),
    })
}
