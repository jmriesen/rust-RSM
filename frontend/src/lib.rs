#![feature(iter_array_chunks)]
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use ir::Routine;
pub mod commands;
pub mod expression;
pub mod external_calls;
pub mod extrinsic_function;
pub mod intrinsic_functions;
pub mod intrinsic_var;
pub mod operators;
pub mod parser;
pub mod variable;
use thiserror::Error;

use crate::parser::routine;
//Introduced to prevent overflows during fuzzing.
//TODO: This is not a perfect solutions, but allows me to keep fuzzing.
const MAX_LINE_LENGTH: usize = 200;
// NOTE: This was pulled out as a function specifically so I could add the mutation skip attribute.
// This should be remove when a long term solution to overflows is found.
#[mutants::skip]
fn check_line_lengths(source_code: &str) -> Result<(), ParsingError> {
    if source_code.lines().any(|x| x.len() > MAX_LINE_LENGTH) {
        Err(ParsingError::HitMaxLineLength)
    } else {
        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ParsingError {
    #[error("Error occurred when tree-sitter parsed the routine")]
    TreeSitterError(()),
    #[error("Quit can only have zero or one argument")]
    QuitExtraArgs(lang_model::Range),
    #[error("Close always takes at least one argument")]
    CloseRequiresArgs(lang_model::Range),
    #[error("If always takes at least one argument")]
    IfRequireArgs(lang_model::Range),
    #[error("not yet supported:{}",.0)]
    NotYetSupported(&'static str),
    #[error("kill exclusive is only supported for local variables with no subscripts")]
    KillExclusiveNonLocal(lang_model::Range),
    #[error(
        "Exceeded max line length {MAX_LINE_LENGTH} TODO: this constraint should be eventually remove. Currently here to prevent stack overflows during fuzzing"
    )]
    HitMaxLineLength,
}

pub trait TreeSitterParser<'a> {
    type NodeType;
    fn new(sitter: &Self::NodeType, source_code: &str) -> Self;
}

pub fn parse_routine(source_code: &str) -> Result<Routine, ParsingError> {
    check_line_lengths(source_code)?;
    routine()
        .parse(source_code)
        .into_result()
        .map_err(|errors| {
            ParsingError::NotYetSupported(
                errors
                    .iter()
                    .map(|error| {
                        let report = Report::build(ReportKind::Error, error.span().into_range())
                            .with_message(error.reason())
                            .with_label(
                                Label::new(error.span().into_range()).with_message(error.reason()),
                            )
                            .finish();
                        report.print(Source::from(source_code)).unwrap();
                        format!("parsing_error:{error},{}", error.span())
                    })
                    .collect::<String>()
                    .leak(),
            )
        })
}

#[cfg(test)]
mod test {

    use crate::{ParsingError, parse_routine};

    #[test]
    fn stack_overflow() {
        //TODO: update with better long term solution (counting how many levels of nesting for
        //example or rewriting to not use nesting.)
        //This test case was found though fuzz testing.
        let source_code = "qq q AAAAAAAAAOAAAAAAAOPFOlAAAAA]AAAA@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ \n";

        assert_eq!(
            parse_routine(source_code).map(|_| () /*I only care about the error case*/),
            Err(ParsingError::HitMaxLineLength),
        )
    }

    #[test]
    #[should_panic]
    #[ignore = "don't have time to track down root cause right now."]
    fn todo_this_should_not_parse() {
        let source_code = "foo k (^A\n";
        let tree = lang_model::create_tree(source_code);
        let _tree = lang_model::type_tree(&tree, source_code)
            .map_err(ParsingError::TreeSitterError)
            .unwrap();
    }
}
