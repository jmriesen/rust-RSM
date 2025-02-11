use ir::operators::{Binary, Unary};

use crate::{Compile, bite_code::BiteCode};

impl Compile for Unary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        bite_code.push(match self {
            Self::Minus => ffi::OPMINUS,
            Self::Plus => ffi::OPPLUS,
            Self::Not => ffi::OPNOT,
        });
    }
}

impl Compile for Binary {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use Binary::*;
        bite_code.push(match self {
            Add => ffi::OPADD,
            Sub => ffi::OPSUB,
            Multiply => ffi::OPMUL,
            Divide => ffi::OPDIV,
            IntDivide => ffi::OPINT,
            Modulus => ffi::OPMOD,
            Power => ffi::OPPOW,
            Concatenate => ffi::OPCAT,
            GreaterThan => ffi::OPGTR,
            And => ffi::OPAND,
            Contains => ffi::OPCON,
            Follows => ffi::OPFOL,
            Equal => ffi::OPEQL,
            LessThan => ffi::OPLES,
            NotEqual => ffi::OPNEQL,
            NotLessThen => ffi::OPNLES,
            NotGreaterThan => ffi::OPNGTR,
            NotAnd => ffi::OPNAND,
            NotContains => ffi::OPNCON,
            NotFollows => ffi::OPNFOL,
            NotSortsAfter => ffi::OPNSAF,
            SortsAfter => ffi::OPSAF,
            Pattern => ffi::OPPAT,
            NotPattern => ffi::OPNPAT,
        });
    }
}
