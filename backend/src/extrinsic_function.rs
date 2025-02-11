use ffi::VAR_U;
use ir::{
    ExtrinsicFunction,
    extrinsic_function::{Args, Location},
};

use super::Compile;
use crate::{bite_code::BiteCode, expression::ExpressionContext, variable::VarContext};
pub enum ExtrinsicFunctionContext {
    Eval,
    Do,
}

impl Compile for ExtrinsicFunction {
    type Context = ExtrinsicFunctionContext;
    fn compile(&self, comp: &mut BiteCode, context: &ExtrinsicFunctionContext) {
        for arg in &self.arguments {
            match arg {
                Args::VarUndefined => comp.push(ffi::VARUNDF),
                Args::ByRef(variable) => {
                    variable.compile(comp, &VarContext::Build);
                    comp.push(ffi::NEWBREF);
                }
                Args::Expression(expression) => expression.compile(comp, &ExpressionContext::Eval),
            }
        }

        //Op code
        comp.push(match self.location {
            Location::Tag(_) => ffi::CMDOTAG,
            Location::Routine(_) => ffi::CMDOROU,
            Location::TagRoutine(_, _) => ffi::CMDORT,
        } as u8);

        // Location
        let (tag, routine): (Option<&str>, Option<&str>) = match &self.location {
            Location::Tag(tag) => (Some(tag), None),
            Location::Routine(routine) => (None, Some(routine)),
            Location::TagRoutine(tag, routine) => (Some(tag), Some(routine)),
        };

        if let Some(routine) = routine {
            let routine: VAR_U = routine.try_into().unwrap();
            comp.extend(routine.as_array().iter().cloned());
        }
        if let Some(tag) = tag {
            let tag: VAR_U = tag.try_into().unwrap();
            comp.extend(tag.as_array().iter().cloned());
        }

        // End marker + number of args
        let marker = match context {
            ExtrinsicFunctionContext::Do => 0,
            ExtrinsicFunctionContext::Eval => 129,
        };
        comp.push(self.arguments.len() as u8 + marker);
    }
}
