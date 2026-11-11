use ir::{
    ExtrinsicFunction,
    extrinsic_function::{Args, Location},
};
use serde::{Deserialize, Serialize};
use symbol_table::Ident;

use super::Compile;
use crate::{
    byte_code::ByteCode,
    expression::ExpressionContext,
    macros::OpCodes,
    runtime::{LegacyEncoding, StackAssembally, TEMP, program_counter::AssemblyDecoder},
    variable::VarContext,
};
#[derive(Clone, Copy)]
pub enum ExtrinsicFunctionContext {
    Eval = 0b10000001,
    Do = 0,
}

OpCodes! {
    ArgmentCodes {
        VarUndefined = 169,
        ByRef = 168,
}
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
                Args::VarUndefined => {
                    comp.push_asm(StackAssembally::ArgmentCodes(ArgmentCodes::VarUndefined))
                }
                Args::ByRef(variable) => {
                    variable.compile(comp, &VarContext::Build);
                    comp.push_asm(StackAssembally::ArgmentCodes(ArgmentCodes::ByRef))
                }
                Args::Expression(expression) => expression.compile(comp, &ExpressionContext::Eval),
            }
        }

        // Location
        let (tag, routine): (Option<&str>, Option<&str>) = match &self.location {
            Location::Tag(tag) => (Some(&tag.0), None),
            Location::Routine(routine) => (None, Some(&routine.0)),
            Location::TagRoutine(tag, routine) => (Some(&tag.0), Some(&routine.0)),
        };
        let routine = routine.map(|ident| Ident::try_from(ident).unwrap());
        let tag = tag.map(|ident| Ident::try_from(ident).unwrap());
        ExtrinsicFunctionCall {
            tag,
            routine,
            //TODO: Context should be passed separably
            number_of_arguments_passed: self.arguments.len() as u8 + (*context as u8),
        }
        .encode(comp);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrinsicFunctionCall {
    pub tag: Option<Ident>,
    routine: Option<Ident>,
    //eval:bool
    pub number_of_arguments_passed: u8,
}

impl LegacyEncoding for ExtrinsicFunctionCall {
    fn decode(decoder: &mut AssemblyDecoder<'_>) -> Option<Self> {
        let r#type = CallCode::decode(decoder)?;
        let (tag, routine) = match r#type {
            CallCode::Tag => (LegacyEncoding::decode(decoder), None),
            CallCode::Routine => (None, LegacyEncoding::decode(decoder)),
            CallCode::TagRoutine => (
                LegacyEncoding::decode(decoder),
                LegacyEncoding::decode(decoder),
            ),
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

        Some(Self {
            tag,
            routine,
            number_of_arguments_passed: args,
        })
    }
    fn encode(&self, byte_code: &mut crate::ByteCode) {
        use crate::byte_code::EncoderExt;
        //Op code
        byte_code.push_raw(match (&self.tag, &self.routine) {
            (Some(_), None) => CallCode::Tag,
            (None, Some(_)) => CallCode::Routine,
            (Some(_), Some(_)) => CallCode::TagRoutine,
            _ => unreachable!(),
        } as u8);

        self.routine.as_ref().map(|x| x.encode(byte_code));
        self.tag.as_ref().map(|x| x.encode(byte_code));
        byte_code.push_raw(self.number_of_arguments_passed);
    }
}
