use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive, Serialize, Deserialize)]
pub enum Unary {
    Minus,
    Plus,
    Not,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive, Serialize, Deserialize)]
pub enum Binary {
    Add,
    Sub,
    Multiply,
    Divide,
    IntDivide,
    Modulus,
    Power,
    Concatenate,
    GreaterThan,
    And,
    Contains,
    Follows,
    Equal,
    LessThan,
    NotEqual,
    NotLessThen,
    NotGreaterThan,
    NotAnd,
    NotContains,
    NotFollows,
    NotSortsAfter,
    SortsAfter,
    Pattern,
    NotPattern,
}
