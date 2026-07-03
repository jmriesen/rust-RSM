use super::PostCondition;
use crate::ExtrinsicFunction;

#[derive(Debug)]
pub enum Do {
    ArgumentLess,
    FunctionCall(Vec<PostCondition<ExtrinsicFunction>>),
}
