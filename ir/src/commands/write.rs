use crate::Expression;

#[derive(Debug, PartialEq, Eq, Clone, derive_visitor::Drive)]
pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}
