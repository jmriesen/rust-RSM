use ir::IntrinsicVar;

use crate::{Compile, bite_code::BiteCode};

impl Compile for IntrinsicVar {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use IntrinsicVar::*;
        bite_code.push(match self {
            Device => ffi::VARD,
            Ecode => ffi::VAREC,
            Estack => ffi::VARES,
            Etrap => ffi::VARET,
            Horolog => ffi::VARH,
            Io => ffi::VARI,
            Job => ffi::VARJ,
            Key => ffi::VARK,
            Principal => ffi::VARP,
            Quit => ffi::VARQ,
            Reference => ffi::VARR,
            Storage => ffi::VARS,
            StackVar => ffi::VARST,
            System => ffi::VARSY,
            Test => ffi::VART,
            X => ffi::VARX,
            Y => ffi::VARY,
        });
    }
}
