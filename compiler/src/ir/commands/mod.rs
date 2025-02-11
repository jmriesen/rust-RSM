pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;

use close::Close;
use r#break::Break;
use r#do::Do;
use r#for::{For, ForKind};
pub use write::Write;

use super::{Compile, Expression};
use crate::bite_code::BiteCode;

pub struct PostCondition<T> {
    condition: Option<Expression>,
    value: T,
}

impl<T, C> Compile for PostCondition<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context) {
        if let Some(condition) = &self.condition {
            bite_code.conditional_jump(condition, |x| self.value.compile(x, context))
        } else {
            self.value.compile(bite_code, context);
        }
    }
}

pub enum Command {
    Write(PostCondition<Vec<Write>>),
    Close(PostCondition<Vec<Close>>),
    Do(PostCondition<Do>),
    Break(PostCondition<Break>),
    Else,
    For(r#for::For),
}
impl Command {
    pub fn new(
        sitter: &lang_model::command,
        source_code: &str,
        line_tail: &mut dyn Iterator<Item = lang_model::command>,
    ) -> Self {
        match sitter.children() {
            lang_model::commandChildren::BrakeCommand(command) => Break::new(&command, source_code),
            lang_model::commandChildren::CloseCommand(command) => Close::new(&command, source_code),
            lang_model::commandChildren::DoCommand(command) => Do::new(&command, source_code),
            lang_model::commandChildren::ElseCommand(_) => Self::Else,
            lang_model::commandChildren::For(command) => {
                Self::For(For::new(&command, source_code, line_tail))
            }
            lang_model::commandChildren::NewCommand(_) => todo!(),
            lang_model::commandChildren::QUITCommand(_) => todo!(),
            lang_model::commandChildren::WriteCommand(command) => Write::new(&command, source_code),
        }
    }
}
impl Compile for Command {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Command::Write(x) => x.compile(bite_code, &()),
            Command::Close(x) => x.compile(bite_code, &()),
            Command::Do(x) => x.compile(bite_code, &()),
            Command::Break(x) => x.compile(bite_code, &()),
            Command::Else => bite_code.push(ffi::OPELSE),
            //TODO I don't like having the explicit return
            //Remove when I restructure the for command
            Command::For(x) => x.compile(bite_code, &()),
        }
        bite_code.push(ffi::OPENDC);
    }
}
