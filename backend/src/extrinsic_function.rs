use ir::{
    ExtrinsicFunction,
    extrinsic_function::{Args, Location},
};

use super::Compile;
use crate::{bite_code::BiteCode, expression::ExpressionContext, variable::VarContext};
#[derive(Clone, Copy)]
pub enum ExtrinsicFunctionContext {
    Eval = 129,
    Do = 0,
}

enum ArgmentCodes {
    VarUndefined = 169 as isize,
    ByRef = 168 as isize,
}

enum CallCode {
    Tag = 141,
    Routine = 142,
    TagRoutine = 143,
}

impl Compile for ExtrinsicFunction {
    type Context = ExtrinsicFunctionContext;
    fn compile(&self, comp: &mut BiteCode, context: &ExtrinsicFunctionContext) {
        for arg in &self.arguments {
            match arg {
                Args::VarUndefined => comp.push(ArgmentCodes::VarUndefined as u8),
                Args::ByRef(variable) => {
                    variable.compile(comp, &VarContext::Build);
                    comp.push(ArgmentCodes::ByRef as u8);
                }
                Args::Expression(expression) => expression.compile(comp, &ExpressionContext::Eval),
            }
        }

        //Op code
        comp.push(match self.location {
            Location::Tag(_) => CallCode::Tag,
            Location::Routine(_) => CallCode::Routine,
            Location::TagRoutine(_, _) => CallCode::TagRoutine,
        } as u8);

        // Location
        let (tag, routine): (Option<&str>, Option<&str>) = match &self.location {
            Location::Tag(tag) => (Some(tag), None),
            Location::Routine(routine) => (None, Some(routine)),
            Location::TagRoutine(tag, routine) => (Some(tag), Some(routine)),
        };
        routine.compile(comp, &());
        tag.compile(comp, &());

        // End marker + number of args
        comp.push(self.arguments.len() as u8 + (*context as u8));
    }
}
