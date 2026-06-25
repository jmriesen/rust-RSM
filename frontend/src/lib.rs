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

#[derive(Error, Debug)]
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
}

pub trait TreeSitterParser<'a> {
    type NodeType;
    fn new(sitter: &Self::NodeType, source_code: &str) -> Self;
}

pub fn parse_routine(source_code: &str) -> Result<Routine, ParsingError> {
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
