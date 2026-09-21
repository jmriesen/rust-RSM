use std::str::FromStr;

use chumsky::prelude::*;
use ir::{
    Expression,
    operators::{Binary, Unary},
};
use value::{Number, Value};

use crate::parser::variable::variable;

use super::Error;
fn str_literal<'src>() -> impl Parser<'src, &'src str, Expression, Error<'src>> {
    none_of("\"")
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(|x| Expression::String(Value::from_str(&x).unwrap()))
        .labelled("String Literal")
        .as_terminal()
}

fn op_u_code<'src>() -> impl Parser<'src, &'src str, Unary, Error<'src>> {
    choice((
        //
        just("+").to(Unary::Plus),
        just("-").to(Unary::Minus),
        just("'").to(Unary::Not),
    ))
}
fn op_b_code<'src>() -> impl Parser<'src, &'src str, Binary, Error<'src>> {
    choice((
        //
        just("+").to(Binary::Add),
        just("-").to(Binary::Sub),
    ))
}
pub fn expression<'src>() -> impl Parser<'src, &'src str, Expression, Error<'src>> {
    recursive(|expr| {
        let atom = choice((
            //Note: must either terminate or move the cursor before a recursive call.
            //To do otherwise will result in infinite recursion.
            expr.clone().delimited_by(just("("), just(")")),
            str_literal(),
            text::int(10).map(|x| Expression::Number(Number::from_str(x).unwrap())),
            variable(expr).map(Expression::Variable).boxed(),
        ))
        .labelled("expression atom")
        .boxed();
        // Handle operator cases.
        // Note: To prevent infinite recursion operators are applied to atoms not expressions.
        // If the first thing we do to parse a binary expression is try and parse another (binary)
        // expression we are in for infinite recursion.
        // `atom` is guarantied to move the cursor before trying to recurs.
        choice((
            //
            //NOTE: This also handles the atom case (no trailing operator + atom) since the first
            //argument to fold matches on zero repetitions
            atom.clone()
                .foldl(
                    op_b_code().then(atom.clone()).repeated(),
                    |lhs, (op, rhs)| Expression::BinaryExpression {
                        left: Box::new(lhs),
                        op_code: op,
                        right: Box::new(rhs),
                    },
                )
                .boxed(),
            op_u_code()
                .repeated()
                .foldr(atom.clone(), |op_code, expression| {
                    Expression::UnaryExpression {
                        op_code,
                        expresstion: Box::new(expression),
                    }
                })
                .boxed(),
        ))
    })
    .labelled("expression")
}
