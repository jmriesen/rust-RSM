use crate::{Expression, Variable};

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Set {
    pub variable: Variable,
    pub value: Expression,
}
