use ir::commands::Command;

use crate::ParsingError;

pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod r#if;
pub mod kill;
pub mod quit;
pub mod set;
pub mod write;

/// Parses a line into a sequence of commands.
pub fn new_line(line: &lang_model::line, source_code: &str) -> Result<Vec<Command>, ParsingError> {
    let mut commands = vec![];

    if !line.level().is_empty() {
        return Err(ParsingError::NotYetSupported("Block indentation"));
    }
    let mut line_tail = line
        .commands()
        .map(|x| x.children())
        .unwrap_or_default()
        .into_iter();

    while let Some(command) = line_tail.next() {
        commands.push(new(&command, source_code, &mut line_tail)?);
    }
    Ok(commands)
}

pub fn new(
    sitter: &lang_model::command,
    source_code: &str,
    line_tail: &mut dyn Iterator<Item = lang_model::command>,
) -> Result<Command, ParsingError> {
    use lang_model::commandChildren as E;
    Ok(match sitter.children() {
        E::BrakeCommand(command) => r#break::new(&command, source_code),
        E::CloseCommand(command) => close::new(&command, source_code)?,
        E::DoCommand(command) => r#do::new(&command, source_code),
        E::ElseCommand(_) => Command::Else,
        E::For(command) => Command::For(r#for::new(&command, source_code, line_tail)?),
        E::NewCommand(_) => Err(ParsingError::NotYetSupported("new command"))?,
        E::QuitCommand(command) => quit::new(&command, source_code)?,
        E::WriteCommand(command) => write::new(&command, source_code),
        E::Set(command) => set::new(&command, source_code),
        E::IfCommand(command) => r#if::new(&command, source_code)?,
        E::KillCommand(command) => kill::new(&command, source_code)?,
    })
}
