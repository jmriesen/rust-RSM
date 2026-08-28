use crate::{
    Compile,
    bite_code::BiteCode,
    extrinsic_function::ExtrinsicFunctionContext,
    runtime::{Decode, OpCode},
};
use ir::commands::r#do::Do;

OpCode! {DoArgLess=145}
impl Compile for Do {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Do::ArgumentLess => bite_code.push(DoArgLess.encode()),
            Do::FunctionCall(x) => x.compile(bite_code, &ExtrinsicFunctionContext::Do),
        }
    }
}
