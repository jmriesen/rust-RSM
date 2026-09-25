use std::fmt::Display;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{Parser, error::RichReason};
use ir::Routine;
pub mod parser;
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

#[derive(Error, Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum ParsingError {
    #[error("Error occurred when tree-sitter parsed the routine")]
    TreeSitterError(()),
    #[error("Quit can only have zero or one argument")]
    QuitExtraArgs,
    #[error("Close always takes at least one argument")]
    CloseRequiresArgs,
    #[error("If always takes at least one argument")]
    IfRequireArgs,
    #[error("Not yet supported:{}",.0)]
    NotYetSupported(&'static str),
    #[error("kill exclusive is only supported for local variables with no subscripts")]
    KillExclusiveNonLocal,
    #[error(
        "Exceeded max line length {MAX_LINE_LENGTH} TODO: this constraint should be eventually remove. Currently here to prevent stack overflows during fuzzing"
    )]
    HitMaxLineLength,
    #[error("First argument must be a variable.")]
    FunctionArgMustBeVariable,
    #[error("Function Expects more arguments")]
    FunctionExpectsMoreArguments,
    #[error("Function Expects Fewer arguments")]
    FunctionExpectsFewerArguments,
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
                        format!("{error},{}", error.span())
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
}
