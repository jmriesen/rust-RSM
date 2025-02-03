use ffi::VAR_U;

use crate::{localvar::VarTypes, ExtrinsicFunctionContext};

//NOTE: I am currently not validating the string size;
pub enum Location {
    Tag(String),
    Routine(String),
    TagRoutine(String, String),
}

pub struct ExtrinsicFunction<'a> {
    location: Location,
    //TODO convert to IR all the way down, and remove lifetime requirement
    arguments: Vec<crate::models::ExtrinsicFunctionArgs<'a>>,
    //NOTE: This affects the compiled output, but I don't think is should.
    //See `compile` for more details.
    contains_paren: bool,
}

impl<'a> ExtrinsicFunction<'a> {
    pub fn new(sitter: &crate::models::ExtrinsicFunction<'a>, source_code: &str) -> Self {
        let tag = sitter.tag().map(|x| {
            let x = x.children();
            use lang_model::TagNameChildren as E;
            let tag = match &x {
                E::NumericIdentifier(x) => x.node(),
                E::identifier(x) => x.node(),
            };
            tag.utf8_text(source_code.as_bytes())
                .expect("tree sitter would have failed in not utf-8")
                .to_owned()
        });
        let routine = sitter.routine().map(|x| {
            x.node()
                .utf8_text(source_code.as_bytes())
                .expect("tree sitter would have failed in not utf-8")
                .to_owned()
        });

        let location = match (tag, routine) {
            (None, Some(routine)) => Location::Routine(routine),
            (Some(tag), None) => Location::Tag(tag),
            (Some(tag), Some(routine)) => Location::TagRoutine(tag, routine),
            (None, None) => unreachable!("Tree-sitter should prevent this"),
        };

        //NOTE It is easier to just remove the trailing VarUndefined when compiling then during parsing
        let mut arguments = sitter.args();
        if arguments
            .last()
            .is_some_and(|x| matches!(x, lang_model::ExtrinsicFunctionArgs::VarUndefined(_)))
        {
            arguments.pop();
        }
        Self {
            arguments,
            location,
            contains_paren: sitter
                .node()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .contains('('),
        }
    }
}

pub fn compile(
    function: &ExtrinsicFunction,
    source_code: &str,
    comp: &mut Vec<u8>,
    context: ExtrinsicFunctionContext,
) {
    use crate::{expression::ExpressionContext, Compileable};

    use lang_model::ExtrinsicFunctionArgs::*;
    for arg in &function.arguments {
        match arg {
            VarUndefined(_) => {
                comp.push(crate::bindings::VARUNDF);
            }
            ByRef(var) => {
                var.children().compile(source_code, comp, VarTypes::Build);
                comp.push(crate::bindings::NEWBREF);
            }
            Expression(exp) => {
                exp.compile(source_code, comp, ExpressionContext::Eval);
            }
        };
    }

    //Op code
    comp.push(match function.location {
        Location::Tag(_) => crate::bindings::CMDOTAG,
        Location::Routine(_) => crate::bindings::CMDOROU,
        Location::TagRoutine(_, _) => crate::bindings::CMDORT,
    } as u8);

    // Location
    let (tag, routine): (Option<&str>, Option<&str>) = match &function.location {
        Location::Tag(tag) => (Some(tag), None),
        Location::Routine(routine) => (None, Some(routine)),
        Location::TagRoutine(tag, routine) => (Some(tag), Some(routine)),
    };

    if let Some(routine) = routine {
        let routine: VAR_U = routine.try_into().unwrap();
        comp.extend(routine.as_array());
    }
    if let Some(tag) = tag {
        let tag: VAR_U = tag.try_into().unwrap();
        comp.extend(tag.as_array());
    }

    // End marker + number of args
    let marker = match context {
        ExtrinsicFunctionContext::Do => {
            //NOTE on line `parse.c:241`
            //args is incremented before we check for ")"
            //Therefor the args value is 1 higher then it should be
            if function.contains_paren {
                1
            } else {
                0
            }
        }
        ExtrinsicFunctionContext::Eval => 129,
    };
    comp.push(function.arguments.len() as u8 + marker);
}
