use super::expression::Expression;

#[derive(Clone)]
pub struct SelectTerm {
    pub condition: Expression,
    pub value: Expression,
}

#[derive(Clone)]
pub struct Function<const REQUIRED: usize, const OPTIONAL: usize> {
    pub required: [Expression; REQUIRED],
    //Note this should really be thought of as a Vec with a fixed capacity but... whatever, this
    //will work, just skip any None values.
    pub optional: [Option<Expression>; OPTIONAL],
}

#[derive(Clone)]
pub struct VarFunction<const REQUIRED: usize, const OPTIONAL: usize> {
    pub variable: super::Variable,
    pub function: Function<REQUIRED, OPTIONAL>,
}

#[derive(Clone)]
pub enum IntrinsicFunction {
    Select {
        terms: Vec<SelectTerm>,
    },
    ///Max number of arguments is 254
    Char {
        args: Vec<Expression>,
    },
    View(Function<2, 2>),
    Ascii(Function<1, 1>),
    Extract(Function<1, 2>),
    Find(Function<2, 1>),
    Fnumber(Function<2, 1>),
    Justify(Function<2, 1>),
    Length(Function<1, 1>),
    Piece(Function<2, 2>),
    Random(Function<1, 0>),
    Reverse(Function<1, 0>),
    Stack(Function<1, 1>),
    Text(Function<1, 0>),
    Translate(Function<2, 1>),

    QLength(VarFunction<0, 0>),
    QSubscript(VarFunction<1, 0>),

    Data(VarFunction<0, 0>),
    Get(VarFunction<0, 1>),
    Increment(VarFunction<0, 1>),

    Name(VarFunction<0, 3>),
    Order(VarFunction<0, 1>),
    Query(VarFunction<0, 1>),
    Next(VarFunction<0, 0>),
}
