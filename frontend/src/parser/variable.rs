use crate::parser::args_list;

use super::Error;
use chumsky::{Parser, prelude::*};
use ir::{
    Expression, Variable,
    variable::{Env, GlobleIdent, UserClassIdentifiers, VariableType},
};
pub fn identifier<'src>() -> impl Parser<'src, &'src str, &'src str, Error<'src>> {
    any()
        .filter(|start: &char| start.is_ascii_alphabetic()|| start == &'%')
        .then(
            any()
                .filter(|middle: &char| middle.is_ascii_alphanumeric())
                .repeated()
                .at_most(32 /*Identifier max size*/ - 1 /* adjustment due to first char being handle separately*/),
        )
        .to_slice()
        .labelled("identifier")
        .as_terminal()
        .as_context()
}

/// WARNING: You must pass in an expression parser.
/// When I initially tried just calling expression in side this function
/// I accidentally causing an infinite loop expression -> variable -> expression -> variable ->...
/// NOTE: these functions are called when setting up the parsers, not when actually
/// parsing.(circular dependencies during parsing is fine, just not during setup)
pub fn variable<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, Variable, Error<'src>> {
    let subscripts = args_list(exp.clone());

    choice((
        //Named
        globle_ident(exp.clone())
            .or_not()
            .then(identifier())
            .map(|(globle_ident, name)| VariableType::Named {
                name: name.to_owned(),
                globle_ident,
            }),
        just("^").to(VariableType::NakedVariable),
        just("@")
            .ignore_then(exp)
            .then_ignore(just("@"))
            .map(Box::new)
            .map(|expression| VariableType::IndirectVariable { expression }),
    ))
    .then(subscripts.or_not())
    .map(|(var_type, subscripts)| Variable {
        var_type,
        subscripts: subscripts.unwrap_or_default(),
    })
    .labelled("Variable")
    .as_non_terminal()
    .as_context()
}

pub fn globle_ident<'src>(
    exp: impl Parser<'src, &'src str, Expression, Error<'src>> + Clone,
) -> impl Parser<'src, &'src str, GlobleIdent, Error<'src>> {
    just("^")
        .ignore_then(
            choice((
                exp.clone()
                    .separated_by(just(","))
                    .at_least(1)
                    .at_most(2)
                    .collect::<Vec<_>>()
                    .delimited_by(just("["), just("]")),
                exp.clone()
                    .separated_by(just(","))
                    .at_least(1)
                    .at_most(2)
                    .collect::<Vec<_>>()
                    .delimited_by(just("|"), just("|")),
            ))
            .or_not(),
        )
        .map(|args| {
            let mut iter = args.into_iter().flatten();
            GlobleIdent {
                user_class: iter.next().map(|uci| {
                    Box::new(UserClassIdentifiers {
                        uci,
                        env: iter.next().map(Env),
                    })
                }),
            }
        })
}
pub fn local_variable_no_subscripts<'src>() -> impl Parser<'src, &'src str, Variable, Error<'src>> {
    identifier()
        .map(|name| Variable {
            var_type: ir::variable::VariableType::Named {
                name: name.to_owned(),
                globle_ident: None,
            },
            subscripts: vec![],
        })
        .labelled("Local variable no subscripts")
        .as_context()
}
