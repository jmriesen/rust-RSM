use ir::operators::{Binary, Unary};

use crate::{Compile, bite_code::BiteCode};

impl Compile for Unary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        bite_code.push(match self {
            Self::Minus => 19,
            Self::Plus => 18,
            Self::Not => 3,
        });
    }
}

pub trait decode: Sized {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])>;
}
impl decode for Unary {
    fn decode(code: u8, tail: &[u8]) -> Option<(Self, &[u8])> {
        match code {
            19 => Some(Self::Minus),
            18 => Some(Self::Plus),
            3 => Some(Self::Not),
            _ => None,
        }
        .map(|x| (x, tail))
    }
}

impl Compile for Binary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use Binary::*;
        bite_code.push(match self {
            Add => 10,
            Sub => 11,
            Multiply => 12,
            Divide => 13,
            IntDivide => 14,
            Modulus => 15,
            Power => 16,
            Concatenate => 17,
            GreaterThan => 22,
            And => 23,
            Contains => 25,
            Follows => 26,
            Equal => 20,
            LessThan => 21,
            NotEqual => 30,
            NotLessThen => 31,
            NotGreaterThan => 32,
            NotAnd => 33,
            NotContains => 35,
            NotFollows => 36,
            NotSortsAfter => 37,
            SortsAfter => 27,
            Pattern => 28,
            NotPattern => 38,
        });
    }
}
