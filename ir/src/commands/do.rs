use super::PostCondition;
use crate::ExtrinsicFunction;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Do {
    ArgumentLess,
    FunctionCall(Vec<PostCondition<ExtrinsicFunction>>),
}
