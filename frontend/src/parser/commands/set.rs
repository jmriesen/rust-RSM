use chumsky::prelude::*;
use ir::commands::{Command, set::Set};

use crate::{
    ParsingError::ParserError,
    parser::{Extra, cmd_p, expression::expression, variable::variable},
};

pub fn set_parser<'src>() -> impl Parser<'src, &'src str, Set, Extra<'src>> {
    cmd_p("set")
        .validate(|x, e, emitter| {
            //TODO: support this.
            if x.is_some() {
                emitter.emit(Rich::custom(
                    e.span(),
                    ParserError("post_conditions of set are not yet supported"),
                ));
            }
        })
        .ignore_then(variable(expression().boxed()))
        .then_ignore(just("="))
        .then(expression())
        .map(|(variable, value)| Set { variable, value })
}

#[cfg(test)]
mod test {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn post_condition() {
        insta::assert_debug_snapshot!(set_parser().parse("s:0 foo=1").into_result())
    }
}
