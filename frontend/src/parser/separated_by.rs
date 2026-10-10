use chumsky::{IterParser, Parser, primitive::any, recovery::via_parser, span::SimpleSpan};

use crate::parser::Extra;

pub fn separated_by_with_recovery<'src, 'b, T: Clone>(
    term: impl Parser<'src, &'src str, T, Extra<'src>>,
    separator: impl Parser<'src, &'src str, (), Extra<'src>> + Clone,
    fallback: impl Fn(SimpleSpan) -> T,
) -> impl Parser<'src, &'src str, Vec<T>, Extra<'src>> {
    term.recover_with(via_parser(
        any()
            .and_is(separator.clone().not())
            .repeated()
            .map_with(move |_, extra| fallback(extra.span())),
    ))
    .separated_by(separator)
    .collect::<Vec<_>>()
}
#[cfg(test)]
mod test {

    use super::*;
    use chumsky::{
        prelude::{Rich, SimpleSpan},
        primitive::just,
    };

    #[test]
    fn no_errors() {
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .parse("a,a,a,a")
                .into_output_errors(),
            (Some(vec!["a", "a", "a", "a"]), vec![]),
        );
    }
    #[test]
    fn unknown_term() {
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .parse("a,b,a,a")
                .into_output_errors(),
            (
                Some(vec!["a", "c", "a", "a"]),
                vec!["found 'b' expected 'a'"],
            ),
        );
        //Longer term
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .parse("a,bbbb,a,a")
                .into_output_errors(),
            (
                Some(vec!["a", "c", "a", "a"]),
                vec!["found 'b' expected 'a'"],
            ),
        );
    }
    #[test]
    fn missing_term() {
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .parse("a,,a,a")
                .into_output_errors(),
            (
                Some(vec!["a", "c", "a", "a"]),
                vec!["found ',' expected 'a'"],
            ),
        );
    }

    fn assert_expected(
        result: (
            Option<Vec<&str>>,
            Vec<Rich<'_, char, SimpleSpan, crate::ParsingError>>,
        ),
        expected: (Option<Vec<&str>>, Vec<&str>),
    ) {
        let (output, errors) = result;
        assert_eq!(
            (
                expected.0,
                expected.1.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            ),
            (
                output,
                errors.iter().map(|x| x.to_string()).collect::<Vec<_>>()
            )
        );
    }
}
