use crate::Expression;

pub enum Break {
    ArgumentLess,
    Arg(Vec<Expression>),
}
