#![feature(array_chunks)]
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod variable;

pub trait TreeSitterParser<'a> {
    type NodeType;
    fn new(sitter: &Self::NodeType, source_code: &str) -> Self;
}
