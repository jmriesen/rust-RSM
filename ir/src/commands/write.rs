use crate::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}
