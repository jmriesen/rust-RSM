pub mod commands;
pub mod expression;
pub use expression::Expression;
pub mod external_calls;
pub use external_calls::ExternalCalls;
pub mod extrinsic_function;
pub use extrinsic_function::ExtrinsicFunction;
pub mod intrinsic_functions;
pub use intrinsic_functions::IntrinsicFunction;
pub mod intrinsic_var;
pub use intrinsic_var::IntrinsicVar;
pub mod operators;
pub mod variable;
pub use variable::Variable;

use crate::bite_code::BiteCode;
trait Compile {
    type Context;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context);
}

impl<T, C> Compile for Vec<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut BiteCode, context: &Self::Context) {
        for term in self {
            term.compile(bite_code, context)
        }
    }
}
