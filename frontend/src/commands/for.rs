use ir::{
    Expression, Variable,
    commands::r#for::{Argument, For, ForKind},
};

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for ForKind {
    type NodeType = lang_model::For<'a>;
    fn new(sitter: &lang_model::For, source_code: &str) -> Self {
        if let Some(var) = sitter.variable() {
            assert!(sitter.args().len() > 0);
            Self::VarLoop {
                variable: Variable::new(&var, source_code),
                arguments: sitter
                    .args()
                    .iter()
                    .map(|x| {
                        let mut iter = x
                            .children()
                            .into_iter()
                            .map(|x| Expression::new(&x, source_code));
                        Argument {
                            start: iter.next().expect("We should always have a starting value"),
                            increment_end: iter.next().map(|x| (x, iter.next())),
                        }
                    })
                    .collect(),
            }
        } else {
            Self::Infinite
        }
    }
}
pub fn new(
    sitter: &lang_model::For,
    source_code: &str,
    line_tail: &mut dyn Iterator<Item = lang_model::command>,
) -> For {
    let kind = ForKind::new(sitter, source_code);
    let mut commands = vec![];
    while let Some(command) = line_tail.next() {
        commands.push(super::new(&command, source_code, line_tail));
    }
    For { kind, commands }
}
