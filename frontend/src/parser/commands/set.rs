use chumsky::prelude::*;
use ir::commands::{PostCondition, set::Set};

use crate::parser::{Extra, cmd_p, expression::expression, variable::variable};

pub fn set_parser<'src>() -> impl Parser<'src, &'src str, PostCondition<Vec<Set>>, Extra<'src>> {
    cmd_p("set")
        .then(
            (variable(expression().boxed())
                .then_ignore(just("="))
                .then(expression())
                .map(|(variable, value)| Set { variable, value }))
            .separated_by(just(","))
            .at_least(1)
            .collect(),
        )
        .map(|(condition, value)| PostCondition { condition, value })
}
