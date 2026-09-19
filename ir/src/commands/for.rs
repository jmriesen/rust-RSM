use crate::{Expression, Variable};

use super::Command;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Argument {
    pub start: Expression,
    pub increment_end: Option<(Expression, Option<Expression>)>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ForKind {
    Infinite,
    VarLoop {
        variable: Variable,
        //TODO insure this vector is none empty
        arguments: Vec<Argument>,
    },
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct For {
    pub kind: ForKind,
    pub commands: Vec<Command>,
}
