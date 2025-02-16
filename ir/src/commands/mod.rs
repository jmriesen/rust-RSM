pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod write;

use r#break::Break;
use close::Close;
use r#do::Do;
use r#for::For;
pub use write::Write;

use super::Expression;

pub struct PostCondition<T> {
    pub condition: Option<Expression>,
    pub value: T,
}

pub enum Command {
    Write(PostCondition<Vec<Write>>),
    Close(PostCondition<Vec<Close>>),
    Do(PostCondition<Do>),
    Break(PostCondition<Break>),
    Else,
    For(For),
}
