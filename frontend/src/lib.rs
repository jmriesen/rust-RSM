#![feature(iter_array_chunks)]
use ir::{Routine, commands::Command};
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod variable;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsingError {
    #[error("Error ocurred when tree-sitter parsed the routine")]
    TreeSitterError(()),
    #[error("Quit can only have zero or one argument")]
    QuitExtraArgs(lang_model::Range),
    #[error("Close always takes at least one argument")]
    CloseRequiresArgs(lang_model::Range),
    #[error("If always takes at least one argument")]
    IfReqiresArgs(lang_model::Range),
    #[error("not yet supported:{}",.0)]
    NotYetSupported(&'static str),
    #[error("kill exclusive is only supported for local variables with no subscripts")]
    KillExclusiveNonLocal(lang_model::Range),
}

pub trait TreeSitterParser<'a> {
    type NodeType;
    fn new(sitter: &Self::NodeType, source_code: &str) -> Self;
}

pub fn parse_routine(source_code: &str) -> Result<Routine, ParsingError> {
    if source_code.lines().any(|x| x.len() > 200) {
        //Introduced to prevent overflows during fuzzing.
        //TODO: This is not a perfect solutions, but allows me to keep fuzzing.
        return Err(ParsingError::NotYetSupported("lines longer then 400 chars"));
    };
    let tree = lang_model::create_tree(source_code);
    let tree = lang_model::type_tree(&tree, source_code).map_err(ParsingError::TreeSitterError)?;

    let tags = tree.children();
    if tags.len() != 1 {
        return Err(ParsingError::NotYetSupported(
            "Only exacly one tag per routine is currenly supported",
        ));
    }
    let block = tags[0]
        .block()
        .ok_or(ParsingError::NotYetSupported("Multiple blocks in one tag"))?;
    let mut lines = vec![];
    for line in block.children() {
        let line = match line {
            lang_model::BlockChildren::line(line) => line,
            lang_model::BlockChildren::Block(_block) => {
                return Err(ParsingError::NotYetSupported("nested blocks"));
            }
        };
        let mut commands = vec![];
        let mut line_tail = line.children().into_iter();
        while let Some(command) = line_tail.next() {
            commands.push(commands::new(&command, source_code, &mut line_tail)?);
        }
        lines.push(commands);
    }
    Ok(lines)
}

pub fn wrap_command_in_routine(source_code: &str) -> Routine {
    let source_code = source_code.replace('\n', "\n ");
    let source_code = &format!("tag {source_code}\n");
    parse_routine(source_code).unwrap()
}

pub fn command_from_source(source_code: &str) -> Command {
    //TODO what is this doing?
    let commands = wrap_command_in_routine(source_code);
    commands
        .into_iter()
        .flatten()
        .next()
        .expect("there should only be one command pressent")
}
#[cfg(test)]
mod test {
    use crate::{ParsingError, parse_routine};

    #[test]
    fn stack_overflow() {
        //TODO: update with better long term solution (counting how many levels of nesting for
        //example or rewriting to not use nesting.)
        let source_code = "qq q AAAAAAAAAOAAAAAAAOPFOlAAAAA]AAAA@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ \n";

        assert_eq!(
            parse_routine(source_code).map(|_| () /*I only care about the error case*/),
            Err(ParsingError::NotYetSupported("lines longer then 200 chars")),
        )
    }

    #[test]
    #[should_panic]
    #[ignore = "don't have time to track down root cause right now."]
    fn todo_this_should_not_parse() {
        let source_code = "foo k (^A\n";
        let tree = lang_model::create_tree(source_code);
        let tree = lang_model::type_tree(&tree, source_code)
            .map_err(ParsingError::TreeSitterError)
            .unwrap();
    }
}
