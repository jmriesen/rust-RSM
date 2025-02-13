use ir::commands::Command;

pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;

pub fn new(
    sitter: &lang_model::command,
    source_code: &str,
    line_tail: &mut dyn Iterator<Item = lang_model::command>,
) -> Command {
    match sitter.children() {
        lang_model::commandChildren::BrakeCommand(command) => r#break::new(&command, source_code),
        lang_model::commandChildren::CloseCommand(command) => close::new(&command, source_code),
        lang_model::commandChildren::DoCommand(command) => r#do::new(&command, source_code),
        lang_model::commandChildren::ElseCommand(_) => Command::Else,
        lang_model::commandChildren::For(command) => {
            Command::For(r#for::new(&command, source_code, line_tail))
        }
        lang_model::commandChildren::NewCommand(_) => todo!(),
        lang_model::commandChildren::QUITCommand(_) => todo!(),
        lang_model::commandChildren::WriteCommand(command) => write::new(&command, source_code),
    }
}
