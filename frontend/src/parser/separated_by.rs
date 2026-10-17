use chumsky::{
    Parser,
    combinator::SeparatedBy,
    primitive::any,
    recovery::{RecoverWith, ViaParser, via_parser},
    span::SimpleSpan,
};

use crate::parser::Extra;

//NOTE: This does not work with allow trailing.
//This is because, it automatically inserts a error on an empty term.
//And trailing is indistinguishable from an empty term.
//as far as I can tell.
#[allow(unused)]
pub fn separated_by_with_recovery<
    'src,
    'b,
    Term: Parser<'src, &'src str, T, Extra<'src>>,
    Seporator: Parser<'src, &'src str, (), Extra<'src>> + Clone,
    T: Clone,
>(
    term: Term,
    separator: Seporator,
    fallback: impl Fn(SimpleSpan) -> T,
) -> SeparatedBy<
    RecoverWith<Term, ViaParser<impl Parser<'src, &'src str, T, Extra<'src>>>>,
    Seporator,
    T,
    (),
    &'src str,
    Extra<'src>,
> {
    term.recover_with(via_parser(
        any()
            .and_is(separator.clone().not())
            .repeated()
            .map_with(move |_, extra| fallback(extra.span())),
    ))
    .separated_by(separator)
}
#[cfg(test)]
mod test {

    use super::*;
    use chumsky::{
        IterParser,
        prelude::{Rich, SimpleSpan},
        primitive::just,
    };

    #[test]
    fn no_errors() {
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .collect()
                .parse("a,a,a,a")
                .into_output_errors(),
            (Some(vec!["a", "a", "a", "a"]), vec![]),
        );
    }
    #[test]
    fn unknown_term() {
        assert_expected(
            separated_by_with_recovery(just("a"), just(",").ignored(), |_| "c")
                .collect()
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
                .collect()
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
                .collect()
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
