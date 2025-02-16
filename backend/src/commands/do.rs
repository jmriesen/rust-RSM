use crate::{Compile, bite_code::BiteCode, extrinsic_function::ExtrinsicFunctionContext};
use ir::commands::r#do::Do;

impl Compile for Do {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Do::ArgumentLess => bite_code.push(ffi::CMDON),
            Do::FunctionCall(x) => x.compile(bite_code, &ExtrinsicFunctionContext::Do),
        }
    }
}
