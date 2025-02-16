use super::{Expression, Variable};

//NOTE: I am currently not validating the string size;
#[derive(Clone)]
pub enum Location {
    Tag(String),
    Routine(String),
    TagRoutine(String, String),
}

#[derive(Clone)]
pub enum Args {
    VarUndefined,
    ByRef(Variable),
    Expression(Expression),
}

#[derive(Clone)]
pub struct ExtrinsicFunction {
    pub location: Location,
    pub arguments: Vec<Args>,
}
