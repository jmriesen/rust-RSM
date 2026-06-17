use ir::{
    Expression,
    commands::{Command, r#if::If},
};
use lang_model::IfCommand;

use crate::TreeSitterParser;

pub fn new(sitter: &IfCommand, source_code: &str) -> Command {
    assert!(
        !sitter.args().is_empty(),
        "If always takes at least one argument"
    );
    Command::If(
        sitter
            .args()
            .iter()
            .map(|x| Expression::new(x, source_code))
            .map(If)
            .collect(),
    )
}
