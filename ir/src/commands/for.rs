use crate::{Expression, Spanned, Variable};

use super::Command;

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Argument {
    pub start: Expression,
    pub increment_end: Option<(Expression, Option<Expression>)>,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum ForKind {
    Infinite,
    VarLoop {
        variable: Spanned<Variable>,
        //TODO insure this vector is none empty
        arguments: Vec<Argument>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct For {
    pub kind: ForKind,
    pub commands: Vec<Spanned<Command>>,
}
