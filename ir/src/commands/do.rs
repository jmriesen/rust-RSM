use super::PostCondition;
use crate::{ExtrinsicFunction, Spanned};

#[derive(Clone, Debug, PartialEq, Eq, derive_visitor::Drive)]
pub enum Do {
    ArgumentLess,
    FunctionCall(Vec<PostCondition<Spanned<ExtrinsicFunction>>>),
}
