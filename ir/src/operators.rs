#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Unary {
    Minus,
    Plus,
    Not,
}

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
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
