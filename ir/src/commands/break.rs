use crate::Expression;

#[derive(Debug)]
pub enum Break {
    ArgumentLess,
    Arg(Vec<Expression>),
}
