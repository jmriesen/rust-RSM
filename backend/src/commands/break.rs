use ir::{Expression, commands::r#break::Break};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext};

impl Compile for Break {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Self::ArgumentLess => {
                bite_code.push(ffi::OPBRK0);
            }
            Self::Arg(args) => {
                for arg in args {
                    arg.compile(bite_code, &ExpressionContext::Eval);
                    if !matches!(arg, Expression::InderectExpression { .. }) {
                        bite_code.push(ffi::OPBRKN);
                    }
                }
            }
        }
    }
}
