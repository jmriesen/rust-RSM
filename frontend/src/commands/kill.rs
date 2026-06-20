use ir::{
    Variable,
    commands::{
        Command,
        kill::{Kill, KillType},
    },
    variable::VariableType,
};
use lang_model::{KillArgChildren, KillCommand};
use std::iter::Peekable;

use crate::TreeSitterParser;

pub fn new(sitter: &KillCommand, source_code: &str) -> Command {
    if sitter.args().is_empty() {
        Command::Kill(vec![Kill {
            r#type: KillType::Exclusive,
            variables: vec![],
        }])
    } else {
        let mut args = sitter.args().into_iter().map(|x| x.children()).peekable();
        let mut groupings = vec![];
        while args.peek_mut().is_some() {
            groupings.extend(
            extract_grouping(source_code, &mut args, |x| {
                matches!(x, KillArgChildren::KillExclusive(_))
            })
            .map(|variables| {
                if variables.iter().all(|x| {
                    x.subscripts.is_empty()
                        && matches!(
                            &x.var_type,
                            VariableType::Named {
                                globle_ident: None,
                                name: _
                            }
                        )
                }) {
                    Kill {
                        r#type: KillType::Exclusive,
                        variables,
                    }
                } else {
                    panic!(
                        "kill exclusive is only supported for local variables with no subscripts"
                    )
                }
            }),
        );
            groupings.extend(
                extract_grouping(source_code, &mut args, |x| {
                    matches!(x, KillArgChildren::KillInclusive(_))
                })
                .map(|variables| Kill {
                    r#type: KillType::Inclusive,
                    variables,
                }),
            );
        }
        Command::Kill(groupings)
    }
}

/// Extract a group of exclusive or inclusive kills
fn extract_grouping<'a>(
    source_code: &str,
    args: &mut Peekable<impl Iterator<Item = KillArgChildren<'a>>>,
    predicate: impl Copy + Fn(&mut KillArgChildren<'a>) -> bool,
) -> Option<Vec<Variable>> {
    let mut variables = vec![];
    while args.peek_mut().is_some_and(predicate) {
        let var = extract_var(args.next().expect("prechecked in loop condition"));
        variables.push(Variable::new(&var, source_code));
    }
    if variables.is_empty() {
        None
    } else {
        Some(variables)
    }
}

fn extract_var(killarg: KillArgChildren<'_>) -> lang_model::Variable<'_> {
    match killarg {
        KillArgChildren::KillExclusive(kill_exclusive) => kill_exclusive.children(),
        KillArgChildren::KillInclusive(kill_inclusive) => kill_inclusive.children(),
    }
}
