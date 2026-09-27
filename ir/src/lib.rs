pub mod commands;
pub mod expression;
pub use expression::Expression;
pub mod external_calls;
pub use external_calls::ExternalCalls;
pub mod extrinsic_function;
pub use extrinsic_function::ExtrinsicFunction;
pub mod intrinsic_functions;
pub use intrinsic_functions::IntrinsicFunction;
pub mod intrinsic_var;
pub use intrinsic_var::IntrinsicVar;
pub mod operators;
pub mod variable;
pub use variable::Variable;

use crate::commands::Command;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    pub inner: T,
    pub start: usize,
    pub end: usize,
}

impl<T: derive_visitor::Drive> derive_visitor::Drive for Spanned<T> {
    fn drive<V: derive_visitor::Visitor>(&self, visitor: &mut V) {
        visitor.visit(self, derive_visitor::Event::Enter);
        self.inner.drive(visitor);
        visitor.visit(self, derive_visitor::Event::Exit);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Tag {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Line {
    pub tag: Option<Spanned<Tag>>,
    pub level: u16,
    pub commands: Vec<Spanned<Command>>,
}

pub type Routine = Vec<Line>;
