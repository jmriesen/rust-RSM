use ir::ExternalCalls;

use crate::{Compile, bite_code::BiteCode};

impl Compile for ExternalCalls {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &()) {
        use ExternalCalls::*;
        bite_code.push(match self {
            Directory => ffi::XCDIR,
            Host => ffi::XCHOST,
            File => ffi::XCFILE,
            ErrMsg => ffi::XCERR,
            OpCom => ffi::XCOPC,
            Signal => ffi::XCSIG,
            Spawn => ffi::XCSPA,
            Version => ffi::XCVER,
            Zwrite => ffi::XCZWR,
            E => ffi::XCE,
            Paschk => ffi::XCPAS,
            V => ffi::XCV,
            XCallX => ffi::XCX,
            Xrsm => ffi::XCXRSM,
            SetEnv => ffi::XCSETENV,
            GetEnv => ffi::XCGETENV,
            RouChk => ffi::XCROUCHK,
            Fork => ffi::XCFORK,
            IC => ffi::XCIC,
            Wait => ffi::XCWAIT,
            Debug => ffi::XCDEBUG,
            Compress => ffi::XCCOMP,
        });
    }
}
