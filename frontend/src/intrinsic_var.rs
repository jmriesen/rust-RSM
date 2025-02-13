use ir::IntrinsicVar;

use crate::TreeSitterParser;

impl<'a> TreeSitterParser<'a> for IntrinsicVar {
    type NodeType = lang_model::IntrinsicVar<'a>;
    fn new(sitter: &lang_model::IntrinsicVar<'a>, _: &str) -> Self {
        use lang_model::IntrinsicVarChildren as S;
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
}
