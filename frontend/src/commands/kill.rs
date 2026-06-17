use ir::{
    Variable,
    commands::{
        Command,
        kill::{Kill, KillExclusive, KillInclusive},
    },
};
use lang_model::{KillArgChildren, KillCommand};
use std::iter::Peekable;

use crate::TreeSitterParser;

pub fn new(sitter: &KillCommand, source_code: &str) -> Command {
    assert!(
        !sitter.args().is_empty(),
        "Close always takes at least one argument"
    );
    let mut args = sitter.args().into_iter().map(|x| x.children()).peekable();
    let mut groupings = vec![];
    while args.peek_mut().is_some() {
        groupings.extend(extract_grouping(
            source_code,
            &mut args,
            |x| matches!(x, KillArgChildren::KillExclusive(_)),
            |x| Kill::Exclusive(KillExclusive(x)),
        ));
        groupings.extend(extract_grouping(
            source_code,
            &mut args,
            |x| matches!(x, KillArgChildren::KillInclusive(_)),
            |x| Kill::Inclusive(KillInclusive(x)),
        ));
    }
    Command::Kill(groupings)
}

/// Extract a group of exclusive or inclusive kills
fn extract_grouping<'a>(
    source_code: &str,
    args: &mut Peekable<impl Iterator<Item = KillArgChildren<'a>>>,
    predicate: impl Copy + Fn(&mut KillArgChildren<'a>) -> bool,
    map: impl Fn(Vec<Variable>) -> Kill,
) -> Option<Kill> {
    let mut variables = vec![];
    while args.peek_mut().is_some_and(predicate) {
        let var = extract_var(args.next().expect("prechecked in loop condition"));
        variables.push(Variable::new(&var, source_code));
    }
    if variables.is_empty() {
        None
    } else {
        Some(map(variables))
    }
}

fn extract_var(killarg: KillArgChildren<'_>) -> lang_model::Variable<'_> {
    match killarg {
        KillArgChildren::KillExclusive(kill_exclusive) => kill_exclusive.children(),
        KillArgChildren::KillInclusive(kill_inclusive) => kill_inclusive.children(),
    }
}

