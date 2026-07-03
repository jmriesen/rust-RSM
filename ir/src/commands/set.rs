use crate::{Expression, Variable};

#[derive(Debug)]
pub struct Set {
    pub variable: Variable,
    pub value: Expression,
}
