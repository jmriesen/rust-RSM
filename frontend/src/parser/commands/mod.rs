use chumsky::prelude::*;
use ir::commands::{Command, set::Set};

use crate::parser::{Extra, cmd_p, expression::expression, variable::variable};

pub mod set;

pub fn set_parser<'src>() -> impl Parser<'src, &'src str, Set, Extra<'src>> {
    cmd_p("set")
        .ignore_then(variable(expression().boxed()))
        .then_ignore(just("="))
        .then(expression())
        .map(|(variable, value)| Set { variable, value })
}

#[cfg(test)]
mod test {
    use super::*;
    use chumsky::{ParseResult, Parser};

    use crate::parser::commands::set_parser;

    #[test]
    fn post_condition() {
        insta::assert_debug_snapshot!(set_parser().parse("s:0 foo=1").into_result())
    }
}
