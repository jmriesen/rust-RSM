pub use byte_code::ByteCode;
use ir::Spanned;

pub mod byte_code;
pub mod commands;
mod conditional_jumps;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
mod line_info;
mod macros;
pub mod operators;
mod routine;
pub use routine::compile_routine;
pub mod runtime;
pub mod value;
pub mod var_u;
pub mod variable;
trait Compile2 {
    type Context;
    //This dose not really need to be a vector it can be a generic sink type.
    //This would allow us to do a streaming serialization if we needed.
    fn compile_new(&self, byte_code: &mut Vec<StackAssembally>, context: &Self::Context);
}

pub trait Compile {
    type Context;
    fn compile(&self, bite_code: &mut ByteCode, context: &Self::Context);
}

impl<T, C> Compile for Vec<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut ByteCode, context: &Self::Context) {
        for term in self {
            term.compile(bite_code, context)
        }
    }
}

impl<T, C> Compile for Option<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut ByteCode, context: &Self::Context) {
        if let Some(inner) = self {
            inner.compile(bite_code, context)
        }
    }
}
impl<T, C> Compile for Spanned<T>
where
    T: Compile,
    T: Compile<Context = C>,
{
    type Context = C;
    fn compile(&self, bite_code: &mut ByteCode, context: &Self::Context) {
        self.inner.compile(bite_code, context)
    }
}
impl<T, C> Compile2 for Vec<T>
where
    T: Compile2,
    T: Compile2<Context = C>,
{
    type Context = C;
    fn compile_new(&self, bite_code: &mut Vec<StackAssembally>, context: &Self::Context) {
        for term in self {
            term.compile_new(bite_code, context)
        }
    }
}

impl<T, C> Compile2 for Option<T>
where
    T: Compile2,
    T: Compile2<Context = C>,
{
    type Context = C;
    fn compile_new(&self, bite_code: &mut Vec<StackAssembally>, context: &Self::Context) {
        if let Some(inner) = self {
            inner.compile_new(bite_code, context)
        }
    }
}
impl<T, C> Compile2 for Spanned<T>
where
    T: Compile2,
    T: Compile2<Context = C>,
{
    type Context = C;
    fn compile_new(&self, bite_code: &mut Vec<StackAssembally>, context: &Self::Context) {
        self.inner.compile_new(bite_code, context)
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub use routine::test::test_compile_command;

use crate::runtime::StackAssembally;
