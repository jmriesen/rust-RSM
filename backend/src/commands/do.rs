use crate::{
    Compile,
    byte_code::ByteCode,
    extrinsic_function::ExtrinsicFunctionContext,
    macros::OpCode,
    runtime::{Decode, StackAssembally},
};
use ir::commands::r#do::Do;

OpCode! {DoArgLess=145}
impl Compile for Do {
    type Context = ();
    fn compile(&self, bite_code: &mut ByteCode, _: &()) {
        match self {
            Do::ArgumentLess => bite_code.push_asm(StackAssembally::DoArgLess(DoArgLess)),
            Do::FunctionCall(x) => x.compile(bite_code, &ExtrinsicFunctionContext::Do),
        }
    }
}
