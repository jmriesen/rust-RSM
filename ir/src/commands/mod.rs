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

use crate::{Spanned, commands::kill::Kill};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostCondition<T> {
    pub condition: Option<Expression>,
    pub value: T,
}

impl<T: derive_visitor::Drive> derive_visitor::Drive for PostCondition<T> {
    fn drive<V: derive_visitor::Visitor>(&self, visitor: &mut V) {
        visitor.visit(self, derive_visitor::Event::Enter);
        self.condition.drive(visitor);
        visitor.visit(self, derive_visitor::Event::Exit);
        self.value.drive(visitor);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Command {
    Write(PostCondition<Vec<Spanned<Write>>>),
    Close(PostCondition<Vec<Close>>),
    Do(PostCondition<Do>),
    Break(PostCondition<Break>),
    Else,
    For(For),
    Set(Set),
    If(Vec<If>),
    Kill(Vec<Kill>),
    Quit(PostCondition<Quit>),
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Quit(pub Option<Expression>);
