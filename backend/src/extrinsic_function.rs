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
    runtime::{LegacyEncoding, StackAssembally, program_counter::AssemblyDecoder},
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
//                       Mask Bits 76543210
const ARGUMENTS_MASK: u8 = /* */ 0b01111111;
const RETURN_EXP_MASK: u8 = /**/ 0b10000000;

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
            number_of_arguments_passed: self.arguments.len() as u8,
            return_value_expected: match context {
                ExtrinsicFunctionContext::Eval => true,
                ExtrinsicFunctionContext::Do => false,
            },
        }
        .encode(comp);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtrinsicFunctionCall {
    pub tag: Option<Ident>,
    routine: Option<Ident>,
    pub number_of_arguments_passed: u8,
    pub return_value_expected: bool,
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
        let return_value_expected = (arg_and_type & RETURN_EXP_MASK) != 0;
        let mut args = arg_and_type & ARGUMENTS_MASK;
        //NOTE: Subtracting one to match C Encoding.
        //I am not sure yet why they did this.
        if return_value_expected {
            args -= 1;
        }
        Some(Self {
            tag,
            routine,
            number_of_arguments_passed: args,
            return_value_expected,
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
        if self.number_of_arguments_passed >= RETURN_EXP_MASK {
            panic!("to many args messing up the mask.")
        }
        let return_value_expected = if self.return_value_expected {
            //NOTE: Subtracting one to match C Encoding.
            //I am not sure yet why they did this.
            RETURN_EXP_MASK + 1
        } else {
            0
        };
        byte_code.push_raw(self.number_of_arguments_passed + return_value_expected);
    }
}
