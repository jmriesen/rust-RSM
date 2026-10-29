use ir::{
    ExtrinsicFunction,
    extrinsic_function::{Args, Location},
};

use super::Compile;
use crate::{
    byte_code::ByteCode,
    expression::ExpressionContext,
    macros::OpCodes,
    runtime::{Decode, program_counter::AssemblyDecoder},
    var_u,
    variable::VarContext,
};
#[derive(Clone, Copy)]
pub enum ExtrinsicFunctionContext {
    Eval = 0b10000001,
    Do = 0,
}

enum ArgmentCodes {
    VarUndefined = 169_isize,
    ByRef = 168_isize,
}

OpCodes!(
    CallCode {
        Tag = 141,
        Routine = 142,
        TagRoutine = 143,
    }
);

impl Compile for ExtrinsicFunction {
    type Context = ExtrinsicFunctionContext;
    fn compile(&self, comp: &mut ByteCode, context: &ExtrinsicFunctionContext) {
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
            Location::Tag(tag) => (Some(&tag.0), None),
            Location::Routine(routine) => (None, Some(&routine.0)),
            Location::TagRoutine(tag, routine) => (Some(&tag.0), Some(&routine.0)),
        };
        routine
            .map(|ident| crate::var_u::Ident::try_from(ident).unwrap())
            .compile(comp, &());
        tag.map(|ident| crate::var_u::Ident::try_from(ident).unwrap())
            .compile(comp, &());

        // End marker + number of args
        comp.push(self.arguments.len() as u8 + (*context as u8));
    }
}

#[derive(Debug)]
pub struct ExtrinsicFunctionCall {
    pub tag: Option<var_u::Ident>,
    #[expect(dead_code)]
    routine: Option<var_u::Ident>,
    //eval:bool
    pub args: u8,
}

impl Decode for ExtrinsicFunctionCall {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let r#type = CallCode::decode(decoder)?;
        let (tag, routine) = match r#type {
            CallCode::Tag => (Decode::decode(decoder), None),
            CallCode::Routine => (None, Decode::decode(decoder)),
            CallCode::TagRoutine => (Decode::decode(decoder), Decode::decode(decoder)),
        };
        assert_eq!(routine, None, "Routine calling not yet supported");
        assert!(tag.is_some(), "default tag calling not yet supported");
        let [arg_and_type] = decoder.consume_n();
        //TODO Check/fix this Note this is not quite the code used to encode.
        //Is there a "hidden/added" argument when in the eval context?
        let _type = if arg_and_type & 0b10000000 != 0 {
            unimplemented!("Eval context do is not yet supported");
            #[expect(unreachable_code)]
            ExtrinsicFunctionContext::Eval
        } else {
            ExtrinsicFunctionContext::Do
        };
        let args = arg_and_type & 0b01111111;

        Some(Self { tag, routine, args })
    }
}
