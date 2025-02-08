pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;

pub use write::Write;

use crate::{bite_code::BiteCode, expression::ExpressionContext};

use super::Expression;

pub enum Command {
    Write {
        condition: Option<Expression>,
        args: Vec<write::Write>,
    },
}
impl Command {
    pub fn new(sitter: &lang_model::command, source_code: &str) -> Option<Self> {
        match sitter.children() {
            lang_model::commandChildren::BrakeCommand(brake_command) => None,
            lang_model::commandChildren::CloseCommand(close_command) => None,
            lang_model::commandChildren::DoCommand(do_command) => None,
            lang_model::commandChildren::ElseCommand(else_command) => None,
            lang_model::commandChildren::For(_) => None,
            lang_model::commandChildren::NewCommand(new_command) => None,
            lang_model::commandChildren::QUITCommand(quitcommand) => None,
            lang_model::commandChildren::WriteCommand(command) => Some(Self::Write {
                condition: command
                    .post_condition()
                    .map(|x| Expression::new(&x, source_code)),
                args: command
                    .args()
                    .iter()
                    .map(|x| Write::new(&x, source_code))
                    .collect(),
            }),
        }
    }
    pub fn compile(&self, comp: &mut BiteCode) {
        match self {
            Command::Write { condition, args } => {
                let post_condition_jump = condition.as_ref().map(|condition| {
                    condition.compile(comp, ExpressionContext::Eval);
                    comp.push(ffi::JMP0);
                    comp.reserve_jump()
                });
                for arg in args {
                    arg.compile(comp)
                }
                if let Some(jump_past) = post_condition_jump {
                    comp.write_jump(jump_past, comp.current_location())
                }
            }
        }
        comp.push(ffi::OPENDC)
    }
}
