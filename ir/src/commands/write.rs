use crate::Expression;

pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}
