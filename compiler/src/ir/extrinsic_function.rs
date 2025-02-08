use ffi::VAR_U;

use super::{expression, variable, Expression, Variable};
use crate::{expression::ExpressionContext, localvar::VarContext, ExtrinsicFunctionContext};

//NOTE: I am currently not validating the string size;
#[derive(Clone)]
pub enum Location {
    Tag(String),
    Routine(String),
    TagRoutine(String, String),
}

#[derive(Clone)]
enum Args {
    VarUndefined,
    ByRef(Variable),
    Expression(Expression),
}
impl Args {
    pub fn new(sitter: &lang_model::ExtrinsicFunctionArgs<'_>, source_code: &str) -> Self {
        use lang_model::ExtrinsicFunctionArgs as E;
        match sitter {
            E::VarUndefined(_) => Self::VarUndefined,
            E::ByRef(var) => Self::ByRef(Variable::new(&var.children(), source_code)),
            E::Expression(exp) => Self::Expression(Expression::new(exp, source_code)),
        }
    }
}

#[derive(Clone)]
pub struct ExtrinsicFunction {
    location: Location,
    //TODO convert to IR all the way down, and remove lifetime requirement
    arguments: Vec<Args>,
    //NOTE: This affects the compiled output, but I don't think is should.
    //See `compile` for more details.
    contains_paren: bool,
}

impl ExtrinsicFunction {
    pub fn new(sitter: &lang_model::ExtrinsicFunction<'_>, source_code: &str) -> Self {
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
        let mut arguments: Vec<_> = sitter
            .args()
            .iter()
            .map(|x| Args::new(x, source_code))
            .collect();
        if arguments
            .last()
            .is_some_and(|x| matches!(x, Args::VarUndefined))
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
    comp: &mut Vec<u8>,
    context: ExtrinsicFunctionContext,
) {
    for arg in &function.arguments {
        match arg {
            Args::VarUndefined => comp.push(ffi::VARUNDF),
            Args::ByRef(variable) => {
                variable::compile(variable, comp, VarContext::Build);
                comp.push(ffi::NEWBREF);
            }
            Args::Expression(expression) => expression.compile(comp, ExpressionContext::Eval),
        }
    }

    //Op code
    comp.push(match function.location {
        Location::Tag(_) => ffi::CMDOTAG,
        Location::Routine(_) => ffi::CMDOROU,
        Location::TagRoutine(_, _) => ffi::CMDORT,
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
