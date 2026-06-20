pub mod r#break;
pub mod close;
pub mod r#do;
pub mod r#for;
pub mod r#if;
pub mod kill;
pub mod set;
pub mod write;

use r#break::Break;
use close::Close;
use r#do::Do;
use r#for::For;
use r#if::If;
use set::Set;
pub use write::Write;

use crate::commands::kill::Kill;

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
    Set(Set),
    If(Vec<If>),
    Kill(Vec<Kill>),
    Quit(PostCondition<Quit>),
}

pub struct Quit(pub Option<Expression>);
