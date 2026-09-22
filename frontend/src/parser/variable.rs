/*
IndirectVariable: $ => seq("@", $.Expression, "@"),
    NakedVariable: $ => "^",
    GlobalVariable: $ => "^",
    GlobalUciVariable: $ => choice(
      seq("^|", $.Expression, "|"),
      //TODO check if square brackets are valid.
      prec(1, seq("^[", $.Expression, "]"))
    ),
    GlobalUciEnvVariable: $ => prec(1, seq("^[", $.Expression, ",", $.Expression, "]")),
    _VariableSubscripts: $ => seq("(", repeatDel($.Expression, ","), ")"),
    Space: $ => " ",


    Variable: $ => choice(
      seq(
        field('heading', choice(
          $.IndirectVariable,
          $.NakedVariable
        )),
        field('subs', $._VariableSubscripts)
      ),
      seq(
        optional(field('heading', choice(
          $.GlobalVariable,
          $.GlobalUciVariable,
          $.GlobalUciEnvVariable
        ))),
        field('name', $.identifier),
        optional(field('subs', $._VariableSubscripts))
      )
    ),
*/

use super::Error;
use chumsky::{Parser, prelude::*};
use ir::{Expression, Variable, variable::VariableType};
pub fn identifier<'src>() -> impl Parser<'src, &'src str, &'src str, Error<'src>> {
    any()
        .filter(|start: &char| start.is_ascii_alphabetic())
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
    exp: impl Parser<'src, &'src str, Expression, Error<'src>>,
) -> impl Parser<'src, &'src str, Variable, Error<'src>> {
    let subscripts = exp
        .separated_by(just(","))
        .collect::<Vec<_>>()
        .delimited_by(just("("), just(")"));
    identifier()
        .then(subscripts.or_not())
        .map(|(name, subscripts)| Variable {
            var_type: VariableType::Named {
                name: name.to_owned(),
                globle_ident: None,
            },
            subscripts: subscripts.unwrap_or_default(),
        })
        .labelled("Variable")
        .as_non_terminal()
        .as_context()
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
