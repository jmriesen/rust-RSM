use ir::operators::{Binary, Unary};

use crate::{
    Compile,
    bite_code::BiteCode,
    runtime::{Decode, Encode, OpCodesForeign},
};

OpCodesForeign! {
    Unary {
        Minus => 19,
        Plus => 18,
        Not => 3,
    }
}

impl Compile for Unary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        bite_code.push(self.encode());
    }
}
impl Compile for Binary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        bite_code.push(self.encode());
    }
}

OpCodesForeign! {
    Binary{
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
        }
}
