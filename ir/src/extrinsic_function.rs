use crate::variable::Ident;

use super::{Expression, Variable};

//NOTE: I am currently not validating the string size;
#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Location {
    Tag(Ident),
    Routine(Ident),
    TagRoutine(Ident, Ident),
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Args {
    VarUndefined,
    ByRef(Variable),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct ExtrinsicFunction {
    pub location: Location,
    pub arguments: Vec<Args>,
}
