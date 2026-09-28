use crate::Expression;

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub struct Close(pub Expression);
