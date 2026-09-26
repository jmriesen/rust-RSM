use crate::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Break {
    ArgumentLess,
    Arg(Vec<Expression>),
}
