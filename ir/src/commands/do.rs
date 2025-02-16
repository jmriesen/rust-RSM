use super::PostCondition;
use crate::ExtrinsicFunction;

pub enum Do {
    ArgumentLess,
    FunctionCall(Vec<PostCondition<ExtrinsicFunction>>),
}
