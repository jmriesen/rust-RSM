use ir::ExternalCalls;

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for ExternalCalls {
    type NodeType = lang_model::XCallCode<'a>;
    fn new(sitter: &lang_model::XCallCode<'a>, _: &str) -> Self {
        use lang_model::XCallCode::*;
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
}
