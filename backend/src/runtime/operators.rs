use ir::operators::{Binary, Unary};
use value::{Number, Value};

pub trait BinaryApply {
    fn apply(&self, first: Value, second: Value) -> Value;
}

impl BinaryApply for Binary {
    fn apply(&self, first: Value, second: Value) -> Value {
        match self {
            Binary::Add => (Number::from(first) + Number::from(second)).into(),
            Binary::Sub => (Number::from(first) - Number::from(second)).into(),
            Binary::Equal => (first == second).into(),
            _ => {
                todo!()
            }
        }
    }
}

pub trait UnaryApply {
    fn apply(&self, value: Value) -> Value;
}

impl UnaryApply for Unary {
    fn apply(&self, value: Value) -> Value {
        let mut num = Number::from(value);
        match self {
            Unary::Minus => {
                num.negate();
                num.into()
            }
            Unary::Plus => num.into(),
            Unary::Not => (!bool::from(&num)).into(),
        }
    }
}
