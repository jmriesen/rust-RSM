use ir::{
    Expression, Variable,
    commands::{Command, set::Set},
};

use crate::TreeSitterParser;

pub fn new(sitter: &lang_model::Set, source_code: &str) -> Command {
    Command::Set(Set {
        variable: Variable::new(&sitter.variable(), source_code),
        value: Expression::new(&sitter.expression(), source_code),
    })
}
