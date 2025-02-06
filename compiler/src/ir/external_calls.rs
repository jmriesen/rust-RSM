pub enum ExternalCalls {
    Directory,
    Host,
    File,
    ErrMsg,
    OpCom,
    Signal,
    Spawn,
    Version,
    Zwrite,
    E,
    Paschk,
    V,
    XCallX,
    Xrsm,
    SetEnv,
    GetEnv,
    RouChk,
    Fork,
    IC,
    Wait,
    Debug,
    Compress,
}

impl ExternalCalls {
    pub fn new(sitter: lang_model::XCallCode) -> Self {
        use crate::models::XCallCode::*;
        match sitter {
            Directory(_) => Self::Directory,
            Host(_) => Self::Host,
            File(_) => Self::File,
            ErrMsg(_) => Self::ErrMsg,
            OpCom(_) => Self::OpCom,
            Signal(_) => Self::Signal,
            Spawn(_) => Self::Spawn,
            Version(_) => Self::Version,
            Zwrite(_) => Self::Zwrite,
            E(_) => Self::E,
            Paschk(_) => Self::Paschk,
            V(_) => Self::V,
            XCallX(_) => Self::XCallX,
            Xrsm(_) => Self::Xrsm,
            SetEnv(_) => Self::SetEnv,
            GetEnv(_) => Self::GetEnv,
            RouChk(_) => Self::RouChk,
            Fork(_) => Self::Fork,
            IC(_) => Self::IC,
            Wait(_) => Self::Wait,
            Debug(_) => Self::Debug,
            Compress(_) => Self::Compress,
        }
    }
    pub fn op_code(&self) -> u8 {
        use ExternalCalls::*;
        match self {
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
        }
    }
}
