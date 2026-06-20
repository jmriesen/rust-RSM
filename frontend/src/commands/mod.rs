use ir::commands::Command;

pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod r#if;
pub mod kill;
pub mod quit;
pub mod set;
pub mod write;

pub fn new(
    sitter: &lang_model::command,
    source_code: &str,
    line_tail: &mut dyn Iterator<Item = lang_model::command>,
) -> Command {
    use lang_model::commandChildren as E;
    match sitter.children() {
        E::BrakeCommand(command) => r#break::new(&command, source_code),
        E::CloseCommand(command) => close::new(&command, source_code),
        E::DoCommand(command) => r#do::new(&command, source_code),
        E::ElseCommand(_) => Command::Else,
        E::For(command) => Command::For(r#for::new(&command, source_code, line_tail)),
        E::NewCommand(_) => todo!(),
        E::QuitCommand(command) => quit::new(&command, source_code),
        E::WriteCommand(command) => write::new(&command, source_code),
        E::Set(command) => set::new(&command, source_code),
        E::IfCommand(command) => r#if::new(&command, source_code),
        E::KillCommand(command) => kill::new(&command, source_code),
    }
}
