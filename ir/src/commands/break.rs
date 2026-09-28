use crate::Expression;

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Break {
    ArgumentLess,
    Arg(Vec<Expression>),
}
