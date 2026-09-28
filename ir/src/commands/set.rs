use crate::{Expression, Spanned, Variable};

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Set {
    pub variable: Spanned<Variable>,
    pub value: Expression,
}
