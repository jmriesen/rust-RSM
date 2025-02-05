pub enum IntrinsicVar {
    Device,
    Ecode,
    Estack,
    Etrap,
    Horolog,
    Io,
    Job,
    Key,
    Principal,
    Quit,
    Reference,
    Storage,
    StackVar,
    System,
    Test,
    X,
    Y,
}

impl IntrinsicVar {
    pub fn new(sitter: lang_model::IntrinsicVar) -> Self {
        use crate::models::IntrinsicVarChildren as S;
        match sitter.children() {
            S::Device(_) => Self::Device,
            S::Ecode(_) => Self::Ecode,
            S::Estack(_) => Self::Estack,
            S::Etrap(_) => Self::Etrap,
            S::Horolog(_) => Self::Horolog,
            S::Io(_) => Self::Io,
            S::Job(_) => Self::Job,
            S::Key(_) => Self::Key,
            S::Principal(_) => Self::Principal,
            S::Quit(_) => Self::Quit,
            S::Reference(_) => Self::Reference,
            S::Storage(_) => Self::Storage,
            S::StackVar(_) => Self::StackVar,
            S::System(_) => Self::System,
            S::Test(_) => Self::Test,
            S::X(_) => Self::X,
            S::Y(_) => Self::Y,
        }
    }
    pub fn op_code(&self) -> u8 {
        use IntrinsicVar::*;
        match self {
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
        }
    }
}
