pub use bite_code::BiteCode;

pub mod bite_code;
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod value;
pub mod variable;

pub trait Compile {
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
