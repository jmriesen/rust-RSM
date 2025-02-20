use crate::{Compile, bite_code::BiteCode, extrinsic_function::ExtrinsicFunctionContext};
use ir::commands::r#do::Do;

const DONE_CODE: u8 = 145;

impl Compile for Do {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        match self {
            Do::ArgumentLess => bite_code.push(DONE_CODE),
            Do::FunctionCall(x) => x.compile(bite_code, &ExtrinsicFunctionContext::Do),
        }
    }
}
