use crate::{Expression, Variable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Set {
    pub variable: Variable,
    pub value: Expression,
}
