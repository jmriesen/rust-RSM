use crate::Expression;

#[derive(Debug)]
pub enum Write {
    Bang,
    Clear,
    Tab(Expression),
    Expression(Expression),
}
