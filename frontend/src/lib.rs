use ariadne::{Label, Report, ReportBuilder, ReportKind, Source};
use chumsky::{
    Parser,
    error::Rich,
    span::{SimpleSpan, SpanWrap},
};
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
    #[error("{}",.0)]
    ParserError(&'static str),
    #[error("Quit can only have zero or one argument")]
    QuitExtraArgs,
    #[error("Close always takes at least one argument")]
    CloseRequiresArgs,
    #[error("If always takes at least one argument")]
    IfRequireArgs,
    #[error("Not yet supported:{}",.0)]
    NotYetSupported(&'static str),
    #[error("Expected local variables with no subscripts")]
    ExpectedLocalVariableWithoutSubscripts,
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

#[cfg(any(test, feature = "fuzzing"))]
pub fn parse_routine_print_errors(
    source_code: &str,
) -> Result<Routine, Vec<Rich<'_, char, SimpleSpan, ParsingError>>> {
    check_line_lengths(source_code).map_err(|x| vec![Rich::custom((0..0).into(), x)])?;
    routine()
        .parse(source_code)
        .into_result()
        .inspect_err(|errors| {
            for error in errors {
                let report = build_report(error);
                report.print(Source::from(source_code)).unwrap();
            }
        })
}
fn build_report<'a>(error: &Rich<'a, char, SimpleSpan, ParsingError>) -> Report<'a> {
    Report::build(ReportKind::Error, error.span().into_range())
        .with_message(error.reason())
        .with_label(Label::new(error.span().into_range()).with_message(error.reason()))
        .finish()
}

#[cfg(test)]
mod test {

    use chumsky::error::RichReason;

    use crate::{ParsingError, parse_routine_print_errors};

    #[test]
    fn stack_overflow() {
        //TODO: update with better long term solution (counting how many levels of nesting for
        //example or rewriting to not use nesting.)
        //This test case was found though fuzz testing.
        let source_code = "qq q AAAAAAAAAOAAAAAAAOPFOlAAAAA]AAAA@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ \n";
        let errors = parse_routine_print_errors(source_code).unwrap_err();
        let errors: Vec<_> = errors.iter().map(|x| x.reason()).collect();

        assert_eq!(
            errors,
            vec![&RichReason::Custom(ParsingError::HitMaxLineLength)]
        )
    }
}
