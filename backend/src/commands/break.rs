use ir::{Expression, commands::r#break::Break};

use crate::{Compile, bite_code::BiteCode, expression::ExpressionContext};
enum BreakCode {
    ArgumentLess = 70,
    Arg = 71,
}

impl Compile for Break {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Self::ArgumentLess => {
                bite_code.push(BreakCode::ArgumentLess as u8);
            }
            Self::Arg(args) => {
                for arg in args {
                    arg.compile(bite_code, &ExpressionContext::Eval);
                    if !matches!(arg, Expression::InderectExpression { .. }) {
                        bite_code.push(BreakCode::Arg as u8);
                    }
                }
            }
        }
    }
}
