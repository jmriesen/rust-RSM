use crate::Expression;

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}
