use crate::{Expression, Variable};

use super::Command;

pub struct Argument {
    pub start: Expression,
    pub increment_end: Option<(Expression, Option<Expression>)>,
}

pub enum ForKind {
    Infinite,
    VarLoop {
        variable: Variable,
        //TODO insure this vector is none empty
        arguments: Vec<Argument>,
    },
}
pub struct For {
    pub kind: ForKind,
    pub commands: Vec<Command>,
}
